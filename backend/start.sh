#!/bin/bash

docker build -t api.countdown.mattdavis.info .
docker run -dp 5347:8000 api.countdown.mattdavis.info
