kcli is a simple Kafka cli helper not require jvm.

for local argument passing through cargo use: cargo run --color=always --package kti --bin kti -- -b localhost:9092 -c kafka-test-input

```mermaid
  graph TD;
      A-->B;
      A-->C;
      B-->D;
      C-->D;
```
