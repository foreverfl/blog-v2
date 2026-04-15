package worker

import (
	"log"
	"runtime/debug"
	"sync"
)

// Pool provides bounded concurrency using a semaphore pattern.
type Pool struct {
	sem chan struct{}
	wg  sync.WaitGroup
}

func NewPool(concurrency int) *Pool {
	return &Pool{
		sem: make(chan struct{}, concurrency),
	}
}

// Submit enqueues a task. It blocks if the pool is at capacity.
func (p *Pool) Submit(fn func()) {
	p.wg.Add(1)
	go func() {
		p.sem <- struct{}{}
		defer func() {
			if r := recover(); r != nil {
				log.Printf("[worker] task panic: %v\n%s", r, debug.Stack())
			}
			<-p.sem
			p.wg.Done()
		}()
		fn()
	}()
}

// Wait blocks until all submitted tasks complete.
func (p *Pool) Wait() {
	p.wg.Wait()
}
