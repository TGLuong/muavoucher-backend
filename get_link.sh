#!/bin/sh

URL=$1

curl_chrome116 -X POST "https://affiliate.shopee.vn/api/v3/gql?q=batchCustomLink" \
-H "affiliate-program-type: 1" \
-H "content-type: application/json; charset=UTF-8" \
-H "origin: https://affiliate.shopee.vn" \
-H "user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36" \
-H "Cookie: $COOKIE" \
-d '{"operationName":"batchGetCustomLink","variables":{"linkParams":[{"originalLink":"'$URL'","advancedLinkParams":{}}],"sourceCaller":"CUSTOM_LINK_CALLER"},"query":"query batchGetCustomLink($linkParams: [CustomLinkParam!], $sourceCaller: SourceCaller){\n     batchCustomLink(linkParams: $linkParams, sourceCaller: $sourceCaller){\n       shortLink\n       longLink\n      failCode\n     }\n} "}'