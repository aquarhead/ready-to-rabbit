# Ready to Rabbit

This is a suite of projects that blocks access to RabbitMQ until it's correctly clustered. It is intended to help rolling update a RabbitMQ cluster. A testing application is also included to help verify the behaviour.

- [`block-the-rabbit`](block-the-rabbit) is a BPF XDP program that blocks the `amqp` port (5672)
- [`ready-the-rabbit`](ready-the-rabbit) is a userspace application that should be run before RabbitMQ is started. It will load the above BPF XDP blocker program and repeatedly check RabbitMQ clustering status, and unload the blocker once the clustering succeed
- [`test-the-rabbit`](test-the-rabbit) is an external (not included in deployments) application that helps to verify no connection should be disrupted during a rolling update of an RabbitMQ cluster
