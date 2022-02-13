.PHONY: create_topics start

create_topics:
	docker exec kafka_hello_world_kafka kafka-topics --create --bootstrap-server localhost:9092 --partitions 1 --replication-factor 1 --topic orders || true;
	docker exec kafka_hello_world_kafka kafka-topics --create --bootstrap-server localhost:9092 --partitions 1 --replication-factor 1 --topic payments || true;
	docker exec kafka_hello_world_kafka kafka-topics --create --bootstrap-server localhost:9092 --partitions 1 --replication-factor 1 --topic shipping || true;
