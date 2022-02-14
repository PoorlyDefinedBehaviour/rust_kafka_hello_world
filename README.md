# TODO: add diagram

## Creating topics

```terminal
make create_topics
```

## Describing topics

```terminal
docker exec -it kafka_hello_world_kafka bash
kafka-topics --describe --topic {{topic_name}} --bootstrap-server localhost:9092
```

## Producing events

```terminal
docker exec -it kafka_hello_world_kafka bash
kafka-console-producer --topic {{topic_name}} --bootstrap-server localhost:9092
```

k## Reading events

```terminal
docker exec -it kafka_hello_world_kafka bash
kafka-console-consumer --topic {{topic_name}} --from-beginning --bootstrap-server localhost:9092
```

kafka-run-class kafka.tools.ConsumerOffsetChecker --topic orders --bootstrap-server localhost:9092
kafka-topics --describe --topic orders --bootstrap-server localhost:9092
kafka-console-producer --topic orders --bootstrap-server localhost:9092
kafka-console-consumer --topic orders --from-beginning --bootstrap-server localhost:9092

curl http://localhost:5000

curl -H "Content-Type: application/json" -X POST -d '{"product_id":"uuid","user_id":"uuid","quantity": 1}' http://localhost:5000/orders
