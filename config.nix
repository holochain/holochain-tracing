{
 # extend the shell with buildInputs specific to this project
 buildInputs = [ ];

 # configure holonix itself
 holonix = {

  # true = use a github repository as the holonix base (recommended)
  # false = use a local copy of holonix (useful for debugging)
  use-github = true;

  # configure the remote holonix github when use-github = true
  github = {

   # can be any github ref
   # branch, tag, commit, etc.
   ref = "v0.0.65";

   # the sha of what is downloaded from the above ref
   # note: even if you change the above ref it will not be redownloaded until
   #       the sha here changes (the sha is the cache key for downloads)
   # note: to get a new sha, get nix to try and download a bad sha
   #       it will complain and tell you the right sha
   sha256 = "1frw8z1d3qdly2lcs7z4liwkkqgb344h7p7n1xzpwaqhhm0xa0kd";

   # the github owner of the holonix repo
   owner = "holochain";

   # the name of the holonix repo
   repo = "holonix";
  };

  # configuration for when use-github = false
  local = {
   # the path to the local holonix copy
   path = ../holonix;
  };

 };

 # configure the release process
 release = {
  hook = {
   # sanity checks before deploying
   # to stop the release
   # exit 1
   preflight = ''
hn-release-hook-preflight-manual
'';

   # bump versions in the repo
   version = ''
hn-release-hook-version-rust
hn-release-hook-version-rust-deps 'holochain_tracing'
'';

   # publish artifacts to the world
   publish = ''
echo "All finished!!!"
'';
  };

  # the commit hash that the release process should target
  # this will always be behind what ends up being deployed
  # the release process needs to add some commits for changelog etc.
  commit = "be57d5fc2f524be3d02c12d8916264745602603d";

  # the semver for prev and current releases
  # the previous version will be scanned/bumped by release scripts
  # the current version is what the release scripts bump *to*
  version = {
   current = "0.0.23";
   previous = "0.0.22";
  };

  github = {
   # markdown to inject into github releases
   # there is some basic string substitution {{ xxx }}
   # - {{ changelog }} will inject the changelog as at the target commit
   template = ''
{{ changelog }}

# Installation

Use Holonix to work with this repository.

See:

- https://github.com/holochain/holonix
- https://nixos.org/
'';

   # owner of the github repository that release are deployed to
   owner = "holochain";

   # repository name on github that release are deployed to
   repo = "holochain-tracing";

   # canonical local upstream name as per `git remote -v`
   upstream = "origin";
  };
 };
}
