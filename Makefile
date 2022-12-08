watch:
	docker-compose -f docker-compose.yml up -d && docker-compose -f docker-compose.yml exec actix_api_test bash

watch-build:
	docker-compose -f docker-compose.yml build

watch-down:
	docker-compose -f docker-compose.yml down