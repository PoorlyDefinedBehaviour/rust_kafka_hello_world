<p align="center">
  <img src="https://user-images.githubusercontent.com/17282221/153785558-7752d0c9-cb0d-4ab1-bba5-693e76e541de.png" />
</p>

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

## Reading events

```terminal
docker exec -it kafka_hello_world_kafka bash
kafka-console-consumer --topic {{topic_name}} --from-beginning --bootstrap-server localhost:9092
```

