package kvrpcpb

import "testing"

func TestGroupAdmissionAdjustmentScore(t *testing.T) {
	for _, scope := range []GroupAdmission_Scope{GroupAdmission_BOTH, GroupAdmission_READ, GroupAdmission_WRITE} {
		for _, score := range []uint32{0, 10, 20, 110, ^uint32(0)} {
			advice := &GroupAdmission{Name: "tenant", AdjustmentScore: score, Scope: scope}
			data, err := advice.Marshal()
			if err != nil {
				t.Fatal(err)
			}
			var decoded GroupAdmission
			if err := decoded.Unmarshal(data); err != nil {
				t.Fatal(err)
			}
			if decoded.GetAdjustmentScore() != score || decoded.GetName() != advice.Name || decoded.GetScope() != scope {
				t.Fatalf("admission advice changed: %v -> %v", advice, &decoded)
			}
		}
	}
}
