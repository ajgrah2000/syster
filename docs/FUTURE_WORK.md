# Future Work

## Event System
- [ ] Event batching for bulk operations
- [ ] Event filtering/middleware for targeted subscriptions
- [ ] Event replay/history for debugging
- [ ] Async event handlers (tokio/async-std)
- [ ] Priority-based listener ordering

## Architecture
- [ ] Split `semantic/events.rs` into folder if >300 lines
- [ ] Metrics/observability layer for EventEmitter

## LSP Features
- [ ] Incremental symbol resolution (fine-grained updates)
- [ ] Workspace-wide event aggregation
- [ ] Snapshot/restore state for crash recovery

## Performance
- [ ] Parallel file population with Rayon
- [ ] Specialized symbol index (trie/inverted index)

## Testing & Quality
- [ ] Property-based testing with proptest
- [ ] Benchmark suite with criterion
- [ ] 100% public API documentation coverage
