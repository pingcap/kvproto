package kvrpcpb

import "testing"

func TestGroupAdmissionAdjustmentScore(t *testing.T) {
	for _, score := range []uint32{0, 10, 20, 110, ^uint32(0)} {
		advice := &GroupAdmission{Name: "tenant", AdjustmentScore: score}
		data, err := advice.Marshal()
		if err != nil {
			t.Fatal(err)
		}
		var decoded GroupAdmission
		if err := decoded.Unmarshal(data); err != nil {
			t.Fatal(err)
		}
		if decoded.GetAdjustmentScore() != score || decoded.GetName() != advice.Name {
			t.Fatalf("admission advice changed: %v -> %v", advice, &decoded)
		}
	}
}
