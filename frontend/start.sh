#!/bin/bash

docker build -t countdown.mattdavis.info .
docker run -dp 4173:4173 countdown.mattdavis.info
