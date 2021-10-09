# foliv protocol

| 名称   | 字节数  | 说明  |
|  ----  | ----  | ----  |
| foliv  | 5 bytes | 固定为'foliv' |
| header_length  | 2 bytes | 协议头的长度 |
| data | header_length bytes | 数据包部分 |

# 数据包部分字段简介

| 序号 | 名称   | 类型  | 说明  |
|  ----  |  ----  | ----  | ----  |
| 1 | UserHash | string | token |
| 2 | Command  | enum | Connect:1/Associate:3/Mux:0x7f |
| 3 | AddressType | enum | IPv4:1/DomainName:3/IPv6:4 |
| 4 | Address | []byte | ipv4长度4，ipv6长度16，domain明文 |
| 5 | Port | uint32 | 目标端口 |
| 6 | SourceName | string | 回源规则名称 |
| 7 | RouterName | string | 路由规则名称（ip,域名黑白名单） |
| 8 | ProcessName | string | 进程名称 |
| 9 | XForwardedFor | [][]byte | x-forwarded-for |
| 10 | IsTouch | bool | 是否为touch |
| 11 | MuxID | uint32 | 多路复用sessionID |
| 12 | Platform | string | 平台名称 |
| 13 | RequestID | []byte | requestID |
| 14 | MemberLevel | uint32 | 会员等级 |

# go-code instruction

```
package main

import (
	"github.com/DoubleCircle-Salt/foliv/go/foliv"
	"github.com/golang/protobuf/proto"
)

func main() {
	f := &foliv.Foliv{
		UserHash:    "dcb45bb4973166c14088cc5815c964086a18ac306561d0d9aa502a4c",
		Command:     foliv.Command_Connect,
		AddressType: foliv.AddressType_DomainName,
		Address:     []byte("www.baidu.com"),
		Port:        443,
		SourceName:  "sourcerule_cn",
	}
	data, err := proto.Marshal(f)
	if err != nil {
		println(err)
		return
	}

	println("length:", len(data))

	newf := &foliv.Foliv{}
	if err := proto.Unmarshal(data, newf); err != nil {
		println(err)
		return
	}

	println("println:", newf.String())
}
```
