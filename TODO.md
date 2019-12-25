* Make Handler: Clone? Send? People would need to implement it themselves, but they can always
  wrap it in an Arc, which we're doing now anyways.
  * Caveeat: If they have counters like usize in there, cloning might just copy them. Is that
    a problem? Not sure, can't be mutable anyways unless wrapped in an additional Mutex
