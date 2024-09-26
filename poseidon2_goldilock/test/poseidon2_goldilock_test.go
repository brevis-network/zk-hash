package test

import (
	"github.com/brevis-network/zk-hash/poseidon2_goldilock"
	"github.com/celer-network/goutils/log"
	"testing"
	"time"
)

func TestPoseidon2Goldilock(t *testing.T) {
	for i := 0; i < 10; i++ {
		go testPoseidon2Parall()
	}
	time.Sleep(10 * time.Second)
}

func testPoseidon2Parall() {
	res := poseidon2_goldilock.ComputePoseidon2GoldilockHash([8]uint64{})
	log.Infof("res: %v", res)
}
