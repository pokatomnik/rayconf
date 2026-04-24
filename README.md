# Rayconf

[XRay](https://github.com/xtls/xray-core) CLI configurations manager.

# The WHY
There are a lot of apps such as [Happ](https://github.com/Happ-proxy/happ-desktop), [v2raytun](https://github.com/mdf45/v2raytun/) and others. They have a nice Graphical UI, but if you prefer the original XRay CLI app you have to manage configurations by yourself.
In common, you have a subscription link or an outbound link like:
- `www.something.com/subscription/abscde` (subscription)
- `vless://94671f42-fdd3-4503-a3e6-bda4a2527560@104.21.22.213:80?encryption=none&security=none&sni=019.electrocellco-cf-019.workers.dev&alpn=http/1.1&fp=chrome&type=ws&host=019.electrocellco-cf-019.workers.dev&path=/eyJqdW5rIjoibUVFb0RQcHJZZSIsInByb3RvY29sIjoidmwiLCJtb2RlIjoicHJveHlpcCIsInBhbmVsSVBzIjpbIjM0LjI1My4yMzQuNjIiXX0=?ed=2560#%5BOpenRay%5D%20%F0%9F%87%A8%F0%9F%87%A6%20CA-11825` (outbound connection link)

And you have to rewrite this config to JSON to use XRay.

To make this process simpler, I made XRay configuration manager called Rayconf

It supports both subscription links and your own connection links.

# Examples:

## Add your server outbound connection

```
$ rayconf add
> vless://4255a5cd-de07-49c5-acb3-39537f1097fb@your-server/...
```

## Add subscription link (base64-encoded supported)
```shell
rayconf remote add
> my_subscription
> www.something.com/subscription/abscde
> plain
```

## Connect to your own server:
```
rayconf select | xray run
> your_server 
```

## Connect to server from subscription:
```
rayconf select --remote | xray run
> my_subscription
> server_from_subscription
```

> Please note, `| xray run` pipes rayconf-made JSON configuration to XRay's stdin (It has such feature)

That's actually It, feel free to as questions. PR's are welcome.

# Thanks
- [XRay](https://github.com/xtls/xray-core) the core thing that makes Internet free
- [AvenCores](https://github.com/AvenCores/goida-vpn-configs) for free VPN configs (Гойда!)
- [Keivan-sf](https://github.com/Keivan-sf) for the original idea, I used his code to get this project done