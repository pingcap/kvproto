package grpctest

import (
	"context"
	"net"
	"testing"
	"time"

	"github.com/gogo/protobuf/proto"
	"github.com/gogo/protobuf/types"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/test/bufconn"
)

type rawProtoCodec struct{}

func (rawProtoCodec) Name() string {
	return "proto"
}

func (rawProtoCodec) Marshal(value interface{}) ([]byte, error) {
	return value.([]byte), nil
}

func (rawProtoCodec) Unmarshal(data []byte, value interface{}) error {
	return proto.Unmarshal(data, value.(proto.Message))
}

func Unmarshal(testingT testing.TB, data []byte, result proto.Message) error {
	testingT.Helper()
	listener := bufconn.Listen(1024 * 1024)
	server := grpc.NewServer()
	server.RegisterService(&grpc.ServiceDesc{
		ServiceName: "test.Decoder",
		HandlerType: (*interface{})(nil),
		Methods: []grpc.MethodDesc{
			{
				MethodName: "Decode",
				Handler: func(_ interface{}, _ context.Context, decode func(interface{}) error, _ grpc.UnaryServerInterceptor) (interface{}, error) {
					return &types.Empty{}, decode(result)
				},
			},
		},
	}, struct{}{})
	defer listener.Close()
	defer server.Stop()
	go server.Serve(listener)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	connection, err := grpc.DialContext(ctx, "passthrough:///decoder",
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithContextDialer(func(ctx context.Context, _ string) (net.Conn, error) {
			return listener.DialContext(ctx)
		}),
		grpc.WithBlock(),
	)
	if err != nil {
		testingT.Fatalf("connect to decoder: %v", err)
	}
	defer connection.Close()
	return connection.Invoke(ctx, "/test.Decoder/Decode", data, &types.Empty{}, grpc.ForceCodec(rawProtoCodec{}))
}
