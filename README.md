# foliv protocol

| 名称   | 字节数  | 说明  |
|  ----  | ----  | ----  |
| foliv  | 5 bytes | 固定为'foliv' |
| header_length  | 4 bytes | 协议头的长度 |
| data | header_length bytes | 数据包部分，具体格式见foliv.proto |

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
		Address:     "www.baidu.com",
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
