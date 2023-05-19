# Design Notes

Lists some research, considerations, and design decisions. It should then be separated into a separate document later.

## Major Goals

- Ease of setup (vs other self-hosted services)
- Low operating cost (vs other managed services)
- API-first

This registry is intended to be a part of the internal platform, rather than as a standalone service. Therefore, certain features such as account management may not be included, but it can still be easily customized.

## Components

- Gateway Worker
- Registry Service Worker: Compatible with [Docker Registry HTTP API v2](https://docs.docker.com/registry/spec/api/)
- Management Service Worker: Crons and HTTP API for mananing service

TBD: What else...?

## Multi-tenancy

There is a tenant object called "repository".

A repository logically isolates where images are stored and specifies access policies to them.

## Image naming convention

The image name is also used as the API base path. It must be specified an appropriate convention for route matching.

The convention: `$host/$repository/$image`

- `$host`: hostname of the worker
- `$repository`: identifier of a repository
- `$image`: identifier of an image

## Storages

We use three types of storage.

- [R2](https://developers.cloudflare.com/r2/api/workers/workers-api-reference/) for blobs
- [KV](https://developers.cloudflare.com/workers/runtime-apis/kv/) for metadata and distributed cache
- [D1](https://developers.cloudflare.com/d1/) for entities (has own lifecycle such as repositories, blob uploads, etc) state

Still, everything runs on Cloudflare.
