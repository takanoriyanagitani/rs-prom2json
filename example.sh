#!/bin/sh

convert_all(){
	cat ./sample.txt |
		./prom2json --all |
		jq -c '{docs, samples}'
}

docs_only(){
	cat ./sample.txt |
		./prom2json --docs |
		jq -c '.'
}

samples_only(){
	cat ./sample.txt |
		./prom2json --samples |
		jq -c '.[]'
}

convert_all
docs_only
samples_only
