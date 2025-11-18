# Origin Development TODO

**Timeline**: 5.5 months (single developer) or 3.5 months (two developers in parallel)
**Target Completion**: April 30, 2026

## Phase 0: Foundation (Week 1) ✅ IN PROGRESS

### Day 1-3: Project Structure
- [x] Initialize Cargo workspace with all crates
- [x] Create comprehensive README with 15 iron rules
- [x] Document 7+1 layer architecture
- [x] Set up directory structure for all modules
- [ ] Create build automation with justfile
- [ ] Set up CI/CD pipeline basics

### Day 4-7: Platform Skeleton
- [ ] Set up wgpu + winit basic skeleton
- [ ] Create platform abstraction layer structure
- [ ] Test hello triangle on desktop (Windows/macOS/Linux)
- [ ] Verify basic rendering pipeline works
- [ ] Document platform-specific quirks

**Checkpoint**: `just build` compiles successfully, hello triangle runs on desktop

---

## Phase 1: Primitive Layer (Week 2-4)

### Week 2: Core Infrastructure
- [ ] Implement mega.wgsl shader foundation
- [ ] Create instance buffer management system
- [ ] Build uniform buffer system for state
- [ ] Set up vertex shader for full-screen triangles

### Week 3: Basic Primitives (Day-by-Day)
- [ ] **Day 1**: RoundedRect primitive with 4-corner radius
- [ ] **Day 2**: Circle primitive with SDF
- [ ] **Day 3**: Line primitive with caps and joins
- [ ] **Day 4**: Arc primitive for circular segments
- [ ] **Day 5**: Path primitive with SDF boolean operations
- [ ] **Day 6-7**: Testing and optimization

### Week 4: Advanced Primitives
- [ ] **Day 1-2**: Text primitive with cosmic-text integration
- [ ] **Day 3**: MSDF texture atlas generation
- [ ] **Day 4-5**: Gradient support (linear, radial, angular)
- [ ] **Day 6**: Shadow effects (inner/outer) via SDF
- [ ] **Day 7**: Blur effects in shader

**Checkpoint**: Render complex UI mockup with all 6 primitives at 120fps

---

## Phase 2: Layout Engine (Week 5-7)

### Week 5: Coordinate System
- [ ] Implement relative coordinate stack
- [ ] Create constraint equation types
- [ ] Build Gauss-Seidel solver for runtime constraints
- [ ] Add parent-relative positioning system

### Week 6: Layout Primitives
- [ ] Implement Column layout
- [ ] Implement Row layout
- [ ] Implement Grid layout
- [ ] Add spacing and padding support
- [ ] Create alignment options (start, center, end, stretch)

### Week 7: Advanced Layout
- [ ] Implement Constraint layout (like iOS AutoLayout)
- [ ] Add aspect ratio constraints
- [ ] Implement flexible sizing (flex-grow, flex-shrink)
- [ ] Window resize handling with layout recomputation
- [ ] Performance optimization (< 1ms for complex layouts)

**Checkpoint**: Login page layout in < 40 lines, auto-adjusts on window resize

---

## Phase 3: Input & Interaction (Week 8-9)

### Week 8: Tag System & Picking
- [ ] Implement u64 tag system (type in high 32, id in low 32)
- [ ] Create RGBA32UI picking render target
- [ ] Build picking buffer readback system
- [ ] Map screen coordinates to tag IDs
- [ ] Handle overlapping elements correctly

### Week 9: Event System
- [ ] Implement hover detection
- [ ] Add press/release events
- [ ] Create drag & drop system
- [ ] Add scroll event handling
- [ ] Implement keyboard input routing
- [ ] Cross-platform input normalization
- [ ] Touch gesture support (pinch, swipe, rotate)

**Checkpoint**: Precise mouse picking on all elements, drag-to-reorder works smoothly

---

## Phase 4: Cache & Optimization (Week 10-11)

### Week 10: State Hashing
- [ ] Implement per-instance state hash (StateHash)
- [ ] Create instance buffer diffing system
- [ ] Build partial buffer update mechanism
- [ ] Track dirty regions for minimal updates

### Week 11: Draw Call Optimization
- [ ] Implement draw call batching/merging
- [ ] Add scissor rectangle optimization for dirty regions
- [ ] Create occlusion culling system
- [ ] Build stress test: 100k dynamic cards
- [ ] Optimize to maintain 120fps at 100k instances
- [ ] Profile and eliminate bottlenecks (target: < 2ms CPU)

**Checkpoint**: 100k animated cards at 120fps, CPU < 2ms per frame

---

## Phase 5: Origin DSL (Week 12-13)

### Week 12: DSL Foundation
- [ ] Design macro syntax (column!, row!, rect!, etc.)
- [ ] Implement declarative UI builder
- [ ] Create property system (width, height, color, etc.)
- [ ] Add callback support for interactions

### Week 13: DSL Refinement
- [ ] Implement component composition
- [ ] Add conditional rendering
- [ ] Create list/iteration support
- [ ] Build state management helpers
- [ ] Write comprehensive examples
- [ ] Ensure developer experience matches/exceeds Makepad

**Checkpoint**: Login page in < 60 lines, developer satisfaction survey ≥ 9/10

---

## Phase 6: Design Platform Adapters (Week 14-17)

### Week 14: Adapter Core Infrastructure
- [ ] Design common IR (Intermediate Representation) format
- [ ] Create adapter trait/interface
- [ ] Build shared primitive mapping layer
- [ ] Implement common texture atlas system
- [ ] Create component/variant abstraction

### Week 15: OriginFigma Parser
- [ ] Integrate Figma REST API client
- [ ] Parse Figma JSON schema completely
- [ ] Extract all visual properties
- [ ] Handle component instances and overrides
- [ ] Process variant tables
- [ ] Extract prototype/interaction data

### Week 16: OriginFigma Visual Mapping
- [ ] Map Rectangle + corners → RoundedRect4
- [ ] Convert Vector boolean ops → SDF combine tree
- [ ] Process Text + Auto Layout → cosmic-text + constraints
- [ ] Implement effects → shader layers
- [ ] Handle gradients, shadows, blurs
- [ ] Ensure pixel-perfect accuracy (≤ 1px error)

### Week 17: OriginFigma Runtime & Tooling
- [ ] Generate .origin binary format
- [ ] Implement runtime constraint solver
- [ ] Add prototype interaction system (click → page transition)
- [ ] Create Figma plugin (TypeScript, 800 lines)
- [ ] Implement WebSocket hot-reload server
- [ ] Build `origin watch` command for live updates
- [ ] Optimize: < 50ms load time for complex files

**Checkpoint**: Drag complex Figma file → origin.exe, instant open, pixel-perfect, interactions work

### Future Adapters (Post-Phase 6)
- [ ] OriginSketch: Sketch file parser and converter
- [ ] OriginXD: Adobe XD adapter
- [ ] OriginPenpot: Penpot open-source adapter
- [ ] Generic adapter API for community extensions

---

## Phase 7: Multi-Platform Build (Week 18)

### Desktop Platforms
- [ ] Windows: wgpu + DX12 backend
- [ ] macOS: wgpu + Metal backend  
- [ ] Linux: wgpu + Vulkan backend
- [ ] Test rendering consistency across all desktop platforms

### Web Platform
- [ ] WebAssembly build configuration
- [ ] wgpu WebGPU backend
- [ ] Browser compatibility testing (Chrome, Firefox, Safari)
- [ ] Optimize bundle size
- [ ] Verify < 100ms cold start

### Mobile Platforms
- [ ] Android: wgpu + Vulkan, APK/AAB packaging
- [ ] iOS: wgpu + Metal, IPA packaging
- [ ] Touch input handling
- [ ] Mobile-specific optimizations

### Build Automation
- [ ] Complete justfile with all platform targets
- [ ] Set up GitHub Actions for all 6 platforms
- [ ] Automated testing on all platforms
- [ ] Release artifact generation

**Checkpoint**: `just <platform>` produces working packages for all 6 platforms

---

## Phase 8: Polish & Release (Week 19-21)

### Week 19: Example Applications
- [ ] Particle-effect login page
- [ ] 3D card flip animation demo
- [ ] 100k dynamic list performance demo
- [ ] E-commerce product gallery
- [ ] Dashboard with real-time charts
- [ ] Complete app template

### Week 20: Documentation & Website
- [ ] Complete API documentation
- [ ] Layer-by-layer implementation guides
- [ ] Video tutorials (design-to-app workflow)
- [ ] origin.ui website with live playground
- [ ] Performance comparison videos (Origin vs Flutter vs React Native)
- [ ] Write 10 technical blog posts

### Week 21: Final Polish
- [ ] Performance profiling and optimization
- [ ] Memory leak detection and fixes
- [ ] Cross-platform consistency verification
- [ ] Accessibility features
- [ ] Error handling and diagnostics
- [ ] Beta testing with real users

**Checkpoint**: Public release ready, all deliverables complete

---

## Post-Launch Roadmap

### Short Term (Q2 2026)
- [ ] Community feedback integration
- [ ] Bug fixes and stability improvements
- [ ] Additional design tool adapters (Sketch, XD, Penpot)
- [ ] Plugin ecosystem foundation
- [ ] Advanced animation system

### Medium Term (Q3-Q4 2026)
- [ ] 3D primitive support
- [ ] Advanced shader effects library
- [ ] Design collaboration features
- [ ] Origin Studio (standalone design tool)
- [ ] Component marketplace

### Long Term (2027+)
- [ ] AI-powered design-to-code
- [ ] Real-time multiplayer collaboration
- [ ] Cloud rendering for complex scenes
- [ ] Extended platform support (embedded, VR/AR)

---

## Critical Success Metrics

Each phase must meet these criteria:

- **Performance**: 120fps maintained under all test scenarios
- **Size**: Binary < 12MB, memory < 60MB
- **Quality**: Pixel error ≤ 1px across platforms
- **Speed**: Design file load < 50ms, web start < 100ms
- **Code Quality**: No trees, no retained mode, pure instance rendering
- **Documentation**: Every feature documented before merge

---

## Team Roles (if parallel development)

### Developer 1: Core Engine (Phases 1-4)
Focuses on rendering, layout, input, and optimization

### Developer 2: High-Level Systems (Phases 5-6)
Focuses on DSL and design platform adapters

### Shared: Phases 0, 7, 8
Both developers work together on foundation, build system, and polish

---

## Risk Management

### High-Risk Items
1. **OriginFigma complexity**: Figma has many edge cases
   - Mitigation: Start with subset, iterate to full coverage
   
2. **Cross-platform rendering consistency**: Subtle differences between backends
   - Mitigation: Automated visual regression tests
   
3. **Performance on mobile**: Lower-power devices
   - Mitigation: Early mobile testing, performance budgets

4. **Binary size**: Feature creep could bloat binary
   - Mitigation: Weekly size checks, aggressive optimization

### Medium-Risk Items
1. **WebGPU browser support**: Not universal yet
   - Mitigation: Fallback to WebGL2 if needed
   
2. **Constraint solver performance**: Complex layouts might be slow
   - Mitigation: Caching, incremental updates

---

## Notes

- This TODO is a living document; adjust timeline based on progress
- Each checkpoint is mandatory before proceeding to next phase
- No shortcuts allowed on the 15 iron rules
- Document all decisions and trade-offs
- Maintain test coverage throughout

**Remember**: Origin — Where design returns to its origin. ⚡
