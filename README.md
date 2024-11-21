# pypi-provenance-auth

This tool follows a similar train of thought as [auth-tarball-from-git] and [backseat-signed],
it uses pypi sigstore attestations as a replacement for signed git tags.

[auth-tarball-from-git]: https://github.com/kpcyrd/auth-tarball-from-git
[backseat-signed]: https://github.com/kpcyrd/backseat-signed

## ⚠️ Missing features ⚠️

This is highly experimental proof-of-concept code:

- [ ] The attestation is not cryptographically verified
- [ ] There is no check the envelope and certificate actually belong to the same attestation
- [ ] Some of the command-line flags are not actually asserted yet

If you're good with sigstore and Rust you're very welcome to contribute!

## Usage

```sh
./pypi-provenance-auth --commit "$(git rev-parse HEAD)" \
    --subject "cryptography-${pkgver}.tar.gz" \
    --attestation-file "./test_data/cryptography-${pkgver}.provenance"
```

## Trivia

This project was started in Stockholm during Hackjunta 2024#2 organized by
[ln5], after a Debian developer ([pabs]) requested my take on PEP-740.

[ln5]: https://github.com/ln5
[pabs]: https://github.com/pabs3

## License

`GPL-3.0-or-later`
