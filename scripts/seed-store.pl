#!/usr/bin/env perl
# seed-store.pl — one-shot bootstrap for beadle's on-disk store.
#
# Reads the current dashboard issue's beadle-state:v1 sentinel + fetches all
# open issues from the target repo via `gh`, and writes an initial state.jsonl
# under store/<target>/. Idempotent: rerunning overwrites the seed file, but
# the Rust enumerator (Phase 2) is what appends going forward.
#
# Usage:
#   scripts/seed-store.pl vsdd-factory
#
# Requires: perl 5.x (macOS default is fine), gh authenticated, JSON::PP core.

use strict;
use warnings;
use JSON::PP;
use Digest::SHA qw(sha256_hex);
use Encode qw(encode_utf8);
use POSIX qw(strftime);

my $target = shift @ARGV or die "usage: $0 <target>\n";
my $intent_path = "targets/$target.intent.yaml";
-r $intent_path or die "no intent manifest at $intent_path\n";

# --- 1. discover target repo + dashboard issue --------------------------------

my $repo = read_intent_field($intent_path, 'repo')
    or die "could not parse target.repo from $intent_path\n";
warn "target: $target · repo: $repo\n";

# Locate the dashboard: search open issues by arcavenai for the sentinel block.
my $dashboard_number = find_dashboard_issue($repo);
warn "dashboard: #$dashboard_number\n";

# --- 2. pull the current sentinel block ---------------------------------------

my $body = gh_json("issue view $dashboard_number --repo $repo --json body")
    ->{body};
my ($sentinel_json) = $body =~ /<!-- beadle-state:v1\s*(.+?)\s*beadle-state -->/s
    or die "no beadle-state:v1 block in dashboard #$dashboard_number\n";
my $sentinel = decode_json($sentinel_json);
warn "sentinel run: $sentinel->{run}, watermark: $sentinel->{watermark}\n";

# --- 3. fetch all open issues (paginated) -------------------------------------

my @issues = gh_paginate(
    "issue list --repo $repo --state open --limit 500 " .
    "--json number,title,author,state,createdAt,updatedAt,closedAt,labels,assignees,body"
);
warn "fetched " . scalar(@issues) . " open issues\n";

# --- 4. write state.jsonl -----------------------------------------------------

my $out_dir = "store/$target";
mkdir "store" unless -d "store";
mkdir $out_dir unless -d $out_dir;
my $out_path = "$out_dir/state.jsonl";
open my $fh, '>', $out_path or die "open $out_path: $!";

my $now = strftime("%Y-%m-%dT%H:%M:%SZ", gmtime);
my $run = $sentinel->{run};

# issue record per open issue
for my $iss (sort { $a->{number} <=> $b->{number} } @issues) {
    my $body_text = $iss->{body} // '';
    my $body_bytes = encode_utf8($body_text);
    my $rec = {
        kind        => "issue",
        ts          => $iss->{updatedAt} || $now,
        target      => $target,
        number      => $iss->{number} + 0,
        observed_in_run => $run,
        title       => $iss->{title},
        author      => ($iss->{author}{login} // 'unknown'),
        state       => lc($iss->{state}),
        created_at  => $iss->{createdAt},
        updated_at  => $iss->{updatedAt},
        closed_at   => $iss->{closedAt},
        labels      => [ map { $_->{name} } @{ $iss->{labels} // [] } ],
        assignees   => [ map { $_->{login} } @{ $iss->{assignees} // [] } ],
        body_len    => length($body_bytes),
        body_sha256 => sha256_hex($body_bytes),
    };
    print $fh canonical_json($rec), "\n";
}

# cluster records from sentinel
my $clusters = $sentinel->{clusters} // {};
for my $cname (sort keys %$clusters) {
    my @members = map { $_ + 0 } @{ $clusters->{$cname} };
    my $rec = {
        kind          => "cluster",
        ts            => $now,
        target        => $target,
        name          => $cname,
        run           => $run,
        members       => \@members,
        last_added_run => $run,  # seed conservatively — mark all as active
        decay         => "active",
    };
    print $fh canonical_json($rec), "\n";
}

# final run record
my $run_rec = {
    kind             => "run",
    ts               => $now,
    target           => $target,
    run              => $run,
    watermark_before => 0,
    watermark_after  => $sentinel->{watermark} + 0,
    counts           => $sentinel->{counts},
    digest           => $sentinel->{digest},
    warmup           => $sentinel->{warmup},
    intent_version   => $sentinel->{intent_version},
    new_this_run     => [ map { $_ + 0 } @{ $sentinel->{new_this_run} // [] } ],
    notes            => "seed record from run-$run sentinel; issue+cluster observations backfilled at seed time",
};
print $fh canonical_json($run_rec), "\n";

close $fh;
warn "wrote $out_path (" . (-s $out_path) . " bytes)\n";

# --- helpers ------------------------------------------------------------------

sub read_intent_field {
    my ($path, $key) = @_;
    open my $ifh, '<', $path or die "open $path: $!";
    while (<$ifh>) {
        if (/^\s*\Q$key\E:\s*(\S+)/) { close $ifh; return $1; }
    }
    close $ifh;
    return undef;
}

sub find_dashboard_issue {
    my ($repo) = @_;
    my $list = gh_json(
        "issue list --repo $repo --state open --author arcavenai " .
        "--search 'beadle Triage Dashboard in:title' --limit 5 --json number,title,body"
    );
    my @hits = grep { $_->{body} =~ /<!-- beadle-state:v1/ } @$list;
    die "found " . scalar(@hits) . " dashboards; expected 1 (STOP for consolidation)\n"
        if @hits != 1;
    return $hits[0]{number};
}

sub gh_json {
    my ($args) = @_;
    my $json = qx(gh $args 2>/dev/null);
    die "gh $args failed (exit $?)\n" if $? != 0;
    return decode_json($json);
}

# `gh issue list --limit N` paginates internally up to N; for our sizes (<500)
# a single call suffices. If a target ever exceeds this, swap in `gh api` with
# `--paginate` on `/repos/{owner}/{repo}/issues`.
sub gh_paginate {
    my ($args) = @_;
    my $result = gh_json($args);
    return ref($result) eq 'ARRAY' ? @$result : ($result);
}

sub canonical_json {
    my ($obj) = @_;
    # JSON::PP with canonical => 1 sorts keys — good enough for deterministic diffing.
    return JSON::PP->new->canonical(1)->utf8(1)->encode($obj);
}
