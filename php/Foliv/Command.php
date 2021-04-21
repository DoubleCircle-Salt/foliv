<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: foliv.proto

namespace Foliv;

use UnexpectedValueException;

/**
 * Protobuf type <code>foliv.Command</code>
 */
class Command
{
    /**
     * Generated from protobuf enum <code>Empty = 0;</code>
     */
    const PBEmpty = 0;
    /**
     * Generated from protobuf enum <code>Connect = 1;</code>
     */
    const Connect = 1;
    /**
     * Generated from protobuf enum <code>Associate = 3;</code>
     */
    const Associate = 3;
    /**
     * Generated from protobuf enum <code>Mux = 127;</code>
     */
    const Mux = 127;

    private static $valueToName = [
        self::PBEmpty => 'PBEmpty',
        self::Connect => 'Connect',
        self::Associate => 'Associate',
        self::Mux => 'Mux',
    ];

    public static function name($value)
    {
        if (!isset(self::$valueToName[$value])) {
            throw new UnexpectedValueException(sprintf(
                    'Enum %s has no name defined for value %s', __CLASS__, $value));
        }
        return self::$valueToName[$value];
    }


    public static function value($name)
    {
        $const = __CLASS__ . '::' . strtoupper($name);
        if (!defined($const)) {
            throw new UnexpectedValueException(sprintf(
                    'Enum %s has no value defined for name %s', __CLASS__, $name));
        }
        return constant($const);
    }
}

