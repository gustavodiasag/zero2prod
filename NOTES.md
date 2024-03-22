# Notes

## Linting

A linter will try to spot unidiomatic code, overly-complex constructs and common mistakes/inefficiencies.

## Async Database Support

> Your database is not sitting next to your application on the same physical machine host: to run queries you have to perform network calls.
> An asynchronous database driver will not reduce how long it takes to process a single query, but it will enable your application to leverage all CPU cores to perform other meaningful work while waiting for the database to return results.
