# Ready to Rabbit

The main application blocks access to RabbitMQ until it's correctly clustered, intended to help rolling update a RabbitMQ cluster. It consists of 2 components:

- a BPF XDP program that blocks the `amqp` port (5672)
- a userspace application that will load the above program and repeatedly check RabbitMQ clustering status, and unload the program once the clustering is deemed

A separate (not included in releases) [testing application](test-the-rabbit) helps to verify no connection should be disrupted during a rolling update of an RabbitMQ cluster.
