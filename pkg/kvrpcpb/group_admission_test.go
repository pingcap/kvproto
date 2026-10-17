package kvrpcpb

import "testing"

func TestGroupAdmissionRoundTrip(t *testing.T) {
	for _, scope := range []GroupAdmission_Scope{GroupAdmission_BOTH, GroupAdmission_READ, GroupAdmission_WRITE} {
		for _, cpu := range [][2]uint64{{0, 0}, {1_000_000, 1_500_000}, {^uint64(0), 1}} {
			groups := &NoisyGroups{
				Names:      []string{"tenant"},
				Advice:     &GroupAdmission{Scope: scope, TargetCpuMicros: cpu[0], CurrentCpuMicros: cpu[1]},
				Background: &GroupAdmission{Scope: scope},
			}
			data, err := groups.Marshal()
			if err != nil {
				t.Fatal(err)
			}
			var decoded NoisyGroups
			if err := decoded.Unmarshal(data); err != nil {
				t.Fatal(err)
			}
			advice := decoded.GetAdvice()
			if advice.GetTargetCpuMicros() != cpu[0] || advice.GetCurrentCpuMicros() != cpu[1] || advice.GetScope() != scope {
				t.Fatalf("advice changed: %v -> %v", groups, &decoded)
			}
			if decoded.GetBackground().GetScope() != scope || len(decoded.GetNames()) != 1 {
				t.Fatalf("message changed: %v -> %v", groups, &decoded)
			}
		}
	}
}
