package benchmark

import "runtime"

type MemoryMonitor struct {
	start uint64
}

func (m *MemoryMonitor) Start() {

	var mem runtime.MemStats

	runtime.GC()

	runtime.ReadMemStats(&mem)

	m.start = mem.Alloc

}

func (m *MemoryMonitor) Stop() float64 {

	var mem runtime.MemStats

	runtime.ReadMemStats(&mem)

	diff := mem.Alloc - m.start

	return float64(diff) /

		(1024 * 1024)

}
