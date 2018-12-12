Fantom RFC's
======================

Written for IETF submission, this is written in a Markdown variant then translated to RFC XML.

To get started, install [mmark](https://github.com/mmarkdown/mmark) and [xml2rfc](https://pypi.org/project/xml2rfc/).

To build the RFC txt files:

```bash
cd rfcs
make txt
```

To build the RFC XML files:

```bash
make xml
```

To build the RFC HTML files:

```bash
make html
```

To parse out just the body and replace the RFC HTML files:

```bash
make html_body
```
