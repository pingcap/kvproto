package kvrpcpb

import "testing"

func TestNoisyGroupsRoundTrip(t *testing.T) {
	for _, held := range []bool{false, true} {
		groups := &NoisyGroups{Names: []string{"tenant", "other"}, BackgroundHeld: held}
		data, err := groups.Marshal()
		if err != nil {
			t.Fatal(err)
		}
		var decoded NoisyGroups
		if err := decoded.Unmarshal(data); err != nil {
			t.Fatal(err)
		}
		if decoded.GetBackgroundHeld() != held || len(decoded.GetNames()) != 2 {
			t.Fatalf("message changed: %v -> %v", groups, &decoded)
		}
	}
}
