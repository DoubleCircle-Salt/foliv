<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: foliv.proto

namespace GPBMetadata;

class Foliv
{
    public static $is_initialized = false;

    public static function initOnce() {
        $pool = \Google\Protobuf\Internal\DescriptorPool::getGeneratedPool();

        if (static::$is_initialized == true) {
          return;
        }
        $pool->internalAddGeneratedFile(
            '
�
foliv.protofoliv"�
Foliv
userHash (	
command (2.foliv.Command\'
addressType (2.foliv.AddressType
address (
port (

sourceName (	

routerName (	
processName (	
xForwardedFor	 (
isTouch
 (
muxID (
platform (	
	requestID (
routerLevel (
	userAgent (	

requestHop (*9
Command	
Empty 
Connect
	Associate
Mux*B
AddressType
InvalidType 
IPv4

DomainName
IPv6B	Z./folivbproto3'
        , true);

        static::$is_initialized = true;
    }
}

