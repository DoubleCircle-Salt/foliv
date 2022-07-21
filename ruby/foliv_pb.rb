# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: foliv.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("foliv.proto", :syntax => :proto3) do
    add_message "foliv.Foliv" do
      optional :userHash, :string, 1
      optional :command, :enum, 2, "foliv.Command"
      optional :addressType, :enum, 3, "foliv.AddressType"
      optional :address, :bytes, 4
      optional :port, :uint32, 5
      optional :sourceName, :string, 6
      optional :routerName, :string, 7
      optional :processName, :string, 8
      repeated :xForwardedFor, :bytes, 9
      optional :isTouch, :bool, 10
      optional :muxID, :uint32, 11
      optional :platform, :string, 12
      optional :requestID, :bytes, 13
      optional :routerLevel, :uint32, 14
      optional :userAgent, :string, 15
      optional :requestHop, :uint32, 16
      optional :appID, :uint32, 17
      optional :peerID, :uint32, 18
      optional :version, :uint32, 19
      optional :roundtripTime, :int32, 20
      repeated :bindIPs, :bytes, 21
      optional :routerPath, :string, 22
    end
    add_enum "foliv.Command" do
      value :Empty, 0
      value :Connect, 1
      value :Associate, 3
      value :Mux, 127
    end
    add_enum "foliv.AddressType" do
      value :InvalidType, 0
      value :IPv4, 1
      value :DomainName, 3
      value :IPv6, 4
    end
  end
end

module Foliv
  Foliv = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("foliv.Foliv").msgclass
  Command = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("foliv.Command").enummodule
  AddressType = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("foliv.AddressType").enummodule
end
