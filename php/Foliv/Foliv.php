<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: foliv.proto

namespace Foliv;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>foliv.Foliv</code>
 */
class Foliv extends \Google\Protobuf\Internal\Message
{
    /**
     * Generated from protobuf field <code>string userHash = 1;</code>
     */
    protected $userHash = '';
    /**
     * Generated from protobuf field <code>.foliv.Command command = 2;</code>
     */
    protected $command = 0;
    /**
     * Generated from protobuf field <code>.foliv.AddressType addressType = 3;</code>
     */
    protected $addressType = 0;
    /**
     * Generated from protobuf field <code>bytes address = 4;</code>
     */
    protected $address = '';
    /**
     * Generated from protobuf field <code>uint32 port = 5;</code>
     */
    protected $port = 0;
    /**
     * Generated from protobuf field <code>string sourceName = 6;</code>
     */
    protected $sourceName = '';
    /**
     * Generated from protobuf field <code>string routerName = 7;</code>
     */
    protected $routerName = '';
    /**
     * Generated from protobuf field <code>string processName = 8;</code>
     */
    protected $processName = '';
    /**
     * Generated from protobuf field <code>repeated bytes xForwardedFor = 9;</code>
     */
    private $xForwardedFor;
    /**
     * Generated from protobuf field <code>bool isTouch = 10;</code>
     */
    protected $isTouch = false;
    /**
     * Generated from protobuf field <code>uint32 muxID = 11;</code>
     */
    protected $muxID = 0;
    /**
     * Generated from protobuf field <code>string platform = 12;</code>
     */
    protected $platform = '';
    /**
     * Generated from protobuf field <code>bytes requestID = 13;</code>
     */
    protected $requestID = '';
    /**
     * Generated from protobuf field <code>uint32 routerLevel = 14;</code>
     */
    protected $routerLevel = 0;
    /**
     * Generated from protobuf field <code>string userAgent = 15;</code>
     */
    protected $userAgent = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $userHash
     *     @type int $command
     *     @type int $addressType
     *     @type string $address
     *     @type int $port
     *     @type string $sourceName
     *     @type string $routerName
     *     @type string $processName
     *     @type string[]|\Google\Protobuf\Internal\RepeatedField $xForwardedFor
     *     @type bool $isTouch
     *     @type int $muxID
     *     @type string $platform
     *     @type string $requestID
     *     @type int $routerLevel
     *     @type string $userAgent
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Foliv::initOnce();
        parent::__construct($data);
    }

    /**
     * Generated from protobuf field <code>string userHash = 1;</code>
     * @return string
     */
    public function getUserHash()
    {
        return $this->userHash;
    }

    /**
     * Generated from protobuf field <code>string userHash = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setUserHash($var)
    {
        GPBUtil::checkString($var, True);
        $this->userHash = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>.foliv.Command command = 2;</code>
     * @return int
     */
    public function getCommand()
    {
        return $this->command;
    }

    /**
     * Generated from protobuf field <code>.foliv.Command command = 2;</code>
     * @param int $var
     * @return $this
     */
    public function setCommand($var)
    {
        GPBUtil::checkEnum($var, \Foliv\Command::class);
        $this->command = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>.foliv.AddressType addressType = 3;</code>
     * @return int
     */
    public function getAddressType()
    {
        return $this->addressType;
    }

    /**
     * Generated from protobuf field <code>.foliv.AddressType addressType = 3;</code>
     * @param int $var
     * @return $this
     */
    public function setAddressType($var)
    {
        GPBUtil::checkEnum($var, \Foliv\AddressType::class);
        $this->addressType = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>bytes address = 4;</code>
     * @return string
     */
    public function getAddress()
    {
        return $this->address;
    }

    /**
     * Generated from protobuf field <code>bytes address = 4;</code>
     * @param string $var
     * @return $this
     */
    public function setAddress($var)
    {
        GPBUtil::checkString($var, False);
        $this->address = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>uint32 port = 5;</code>
     * @return int
     */
    public function getPort()
    {
        return $this->port;
    }

    /**
     * Generated from protobuf field <code>uint32 port = 5;</code>
     * @param int $var
     * @return $this
     */
    public function setPort($var)
    {
        GPBUtil::checkUint32($var);
        $this->port = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>string sourceName = 6;</code>
     * @return string
     */
    public function getSourceName()
    {
        return $this->sourceName;
    }

    /**
     * Generated from protobuf field <code>string sourceName = 6;</code>
     * @param string $var
     * @return $this
     */
    public function setSourceName($var)
    {
        GPBUtil::checkString($var, True);
        $this->sourceName = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>string routerName = 7;</code>
     * @return string
     */
    public function getRouterName()
    {
        return $this->routerName;
    }

    /**
     * Generated from protobuf field <code>string routerName = 7;</code>
     * @param string $var
     * @return $this
     */
    public function setRouterName($var)
    {
        GPBUtil::checkString($var, True);
        $this->routerName = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>string processName = 8;</code>
     * @return string
     */
    public function getProcessName()
    {
        return $this->processName;
    }

    /**
     * Generated from protobuf field <code>string processName = 8;</code>
     * @param string $var
     * @return $this
     */
    public function setProcessName($var)
    {
        GPBUtil::checkString($var, True);
        $this->processName = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>repeated bytes xForwardedFor = 9;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getXForwardedFor()
    {
        return $this->xForwardedFor;
    }

    /**
     * Generated from protobuf field <code>repeated bytes xForwardedFor = 9;</code>
     * @param string[]|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setXForwardedFor($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::BYTES);
        $this->xForwardedFor = $arr;

        return $this;
    }

    /**
     * Generated from protobuf field <code>bool isTouch = 10;</code>
     * @return bool
     */
    public function getIsTouch()
    {
        return $this->isTouch;
    }

    /**
     * Generated from protobuf field <code>bool isTouch = 10;</code>
     * @param bool $var
     * @return $this
     */
    public function setIsTouch($var)
    {
        GPBUtil::checkBool($var);
        $this->isTouch = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>uint32 muxID = 11;</code>
     * @return int
     */
    public function getMuxID()
    {
        return $this->muxID;
    }

    /**
     * Generated from protobuf field <code>uint32 muxID = 11;</code>
     * @param int $var
     * @return $this
     */
    public function setMuxID($var)
    {
        GPBUtil::checkUint32($var);
        $this->muxID = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>string platform = 12;</code>
     * @return string
     */
    public function getPlatform()
    {
        return $this->platform;
    }

    /**
     * Generated from protobuf field <code>string platform = 12;</code>
     * @param string $var
     * @return $this
     */
    public function setPlatform($var)
    {
        GPBUtil::checkString($var, True);
        $this->platform = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>bytes requestID = 13;</code>
     * @return string
     */
    public function getRequestID()
    {
        return $this->requestID;
    }

    /**
     * Generated from protobuf field <code>bytes requestID = 13;</code>
     * @param string $var
     * @return $this
     */
    public function setRequestID($var)
    {
        GPBUtil::checkString($var, False);
        $this->requestID = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>uint32 routerLevel = 14;</code>
     * @return int
     */
    public function getRouterLevel()
    {
        return $this->routerLevel;
    }

    /**
     * Generated from protobuf field <code>uint32 routerLevel = 14;</code>
     * @param int $var
     * @return $this
     */
    public function setRouterLevel($var)
    {
        GPBUtil::checkUint32($var);
        $this->routerLevel = $var;

        return $this;
    }

    /**
     * Generated from protobuf field <code>string userAgent = 15;</code>
     * @return string
     */
    public function getUserAgent()
    {
        return $this->userAgent;
    }

    /**
     * Generated from protobuf field <code>string userAgent = 15;</code>
     * @param string $var
     * @return $this
     */
    public function setUserAgent($var)
    {
        GPBUtil::checkString($var, True);
        $this->userAgent = $var;

        return $this;
    }

}

