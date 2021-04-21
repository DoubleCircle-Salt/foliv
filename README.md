# foliv protocol

| 名称   | 字节数  | 说明  |
|  ----  | ----  | ----  |
| foliv  | 5 bytes | 固定为'foliv' |
| header_length  | 4 bytes | 协议头的长度 |
｜data | header_length bytes | 数据包部分，具体格式见foliv.proto |
