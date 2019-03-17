# TACRON

[![Build Status](https://travis-ci.org/letaron/tacron.svg?branch=master)](https://travis-ci.org/letaron/tacron)

!! WIP !!

Manage cron jobs from different sources.

---

## Goal

Cron management can be a tiedous task. TaCron help you by using crons jobs for sources you own.

## Installation

```bash
make init
```

## Configuration

```bash
vi config.yaml
```

## Usage

Start TaCron:

```bash
make run
```

### Refresh TaCron

You can ask TaCron to refresh it's config by issuing a `SIGHUP` signal, ie:
```bash
pkill -SIGHUP tacron
```
