SHELL := /bin/bash
OUT=dist/

build:
	dart compile exe src/main.dart -o ./dist/imgutils

install: build
	sudo mv dist/imgutils /usr/local/bin/imgutils

format:
	dart format ./src
