package kvrpcpb

import "testing"

func TestGroupAdmissionAdjustmentPresence(t *testing.T) {
	for _, advice := range []*GroupAdmission{
		{Name: "tenant", AdmitPermille: 500},
		{Name: "tenant", AdmitPermille: 1000, Adjustment: &GroupAdmission_AdjustmentScore{AdjustmentScore: 0}},
		{Name: "tenant", AdmitPermille: 1000, Adjustment: &GroupAdmission_AdjustmentScore{AdjustmentScore: 20}},
	} {
		data, err := advice.Marshal()
		if err != nil {
			t.Fatal(err)
		}
		var decoded GroupAdmission
		if err := decoded.Unmarshal(data); err != nil {
			t.Fatal(err)
		}
		if (decoded.Adjustment == nil) != (advice.Adjustment == nil) || decoded.GetAdjustmentScore() != advice.GetAdjustmentScore() {
			t.Fatalf("score presence or value changed: %v -> %v", advice, &decoded)
		}
	}
}
