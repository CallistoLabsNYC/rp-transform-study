# Redpanda Transforms Study
Building a Message Queue using Redpanda Transforms

## Premise
Build a high-throughput Analytics System using Redpanda as a Message Queue.

We will make a simple message queue that exists as 1 redpanda topic called `queue`. It has 1 partition and will run 2 transforms.

These two transforms each will match incoming messages on a their designated data type. If there is a match, it will insert the data into the correct database table.

We are simulating a webpage analytics with our two message type: `PageView` and `PageEvent`.