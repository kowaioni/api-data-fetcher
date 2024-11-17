#!/bin/bash

cd backend
cargo run &

cd ../frontend
npx http-server public -p 8080
