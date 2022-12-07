watch:
	docker-compose -f docker-compose.yaml up -d && docker-compose -f docker-compose.yaml exec actix_api_test bash

watch-build:
	docker-compose -f docker-compose.yaml build

watch-down:
	docker-compose -f docker-compose.yaml down