
bad-hashish: a tool for recursively, remotely multi-hashing files

"recursively" meaning that files inside archives (.zip, .tar.gz) are hashed
without extracting everything to disk.

"remotely" meaning that large remote (HTTP/HTTPS) files can be hashed in a
streaming fashion without saving to disk.

"multi-" meaning that mulitple hash algorithms are computed in a single pass.

There are other ways to do most of these; in un-UNIX-y fashion (for now) this
tool does them all together.

## Planned Features

- sha1, sha256, sha512, md5, blake2b
- support base64, base32, hex (upper/lower), etc
- can recurse on .tar and .zip (and more?) without hitting disk
- can stream files via HTTP(S) without hitting disk
- variable output (json, tsv, etc)

Someday?

- dat, ipfs, zsync index computations
- simhash/minhash/etc, for plain text
  https://github.com/bartolsthoorn/simhash-rs
- support piping out to arbitary other commands
  (eg, for pdf extraction simhash, image hash...)
  https://github.com/abonander/img_hash

## Planned Libraries

rust:
- zip
- tar + flate2
- tree_magic
- rust-crypto
- crc
- clap
- error-chain
- reqwest
- log (or slog?)
- rayon (for parallelization?)
- something json
- csv (xsv?)
- data-encoding

## Minimum Viable Version

Parse arguments as local files or URLs. Either way, start reading/streaming
data and hand off pipe to a thing that consumes 4MB chunks at a time and
hashes.

Next, add parallelization (rayon?) for hashes.

Output as space-separated (default), csv, or json, one line per file.

Examples:

    hashish some_file.txt

    cat zip_urls.txt | parallel -j8 hashish --recurse-only {} > all_hashes.txt

Arguments:
- chunk size
- recurse into files or not
- output format
- cores to use?

## Later Thoughts

Limited by {CPU, disk, network}? Where to parallelize? Data locality.
