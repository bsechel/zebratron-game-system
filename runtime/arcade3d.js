import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { ZebratronCartridgeSystem } from './src/index.js';

class Arcade3DDemo {
    constructor() {
        this.scene = null;
        this.camera = null;
        this.renderer = null;
        this.controls = null;
        this.arcadeMachine = null;
        this.screenTexture = null;
        this.system = null;
        this.isRunning = false;
        this.currentCartridge = null;
        this.activeKeys = new Set();
        
        this.initThreeJS();
        this.createArcadeMachine();
        this.setupUI();
        this.setupKeyboardListeners();
        this.initializeZGS();
        this.animate();
    }

    initThreeJS() {
        // Scene setup
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0x111111);

        // Camera setup
        this.camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        this.camera.position.set(0, 1.5, 3);

        // Renderer setup
        this.renderer = new THREE.WebGLRenderer({ antialias: true });
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        this.renderer.shadowMap.enabled = true;
        this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
        document.getElementById('container').appendChild(this.renderer.domElement);

        // Controls
        this.controls = new OrbitControls(this.camera, this.renderer.domElement);
        this.controls.enableDamping = true;
        this.controls.dampingFactor = 0.05;
        this.controls.target.set(0, 1.2, 0);

        // Lighting
        const ambientLight = new THREE.AmbientLight(0x404040, 0.3);
        this.scene.add(ambientLight);

        const spotLight = new THREE.SpotLight(0x00ff88, 1, 10, Math.PI / 4, 0.5);
        spotLight.position.set(0, 4, 2);
        spotLight.castShadow = true;
        this.scene.add(spotLight);

        const screenLight = new THREE.PointLight(0x4488ff, 0.8, 5);
        screenLight.position.set(0, 1.2, 0.5);
        this.scene.add(screenLight);

        // Floor
        const floorGeometry = new THREE.PlaneGeometry(10, 10);
        const floorMaterial = new THREE.MeshLambertMaterial({ color: 0x222222 });
        const floor = new THREE.Mesh(floorGeometry, floorMaterial);
        floor.rotation.x = -Math.PI / 2;
        floor.receiveShadow = true;
        this.scene.add(floor);

        // Window resize handler
        window.addEventListener('resize', () => this.onWindowResize(), false);
    }

    createArcadeMachine() {
        this.arcadeMachine = new THREE.Group();

        // Main cabinet body
        const cabinetGeometry = new THREE.BoxGeometry(0.8, 1.8, 0.6);
        const cabinetMaterial = new THREE.MeshLambertMaterial({ color: 0x333333 });
        const cabinet = new THREE.Mesh(cabinetGeometry, cabinetMaterial);
        cabinet.position.y = 0.9;
        cabinet.castShadow = true;
        this.arcadeMachine.add(cabinet);

        // Screen bezel
        const bezelGeometry = new THREE.BoxGeometry(0.5, 0.4, 0.05);
        const bezelMaterial = new THREE.MeshLambertMaterial({ color: 0x111111 });
        const bezel = new THREE.Mesh(bezelGeometry, bezelMaterial);
        bezel.position.set(0, 1.3, 0.3);
        this.arcadeMachine.add(bezel);

        // Create screen texture (320x240 RGBA)
        const screenData = new Uint8Array(320 * 240 * 4);
        // Fill with dark blue initially
        for (let i = 0; i < screenData.length; i += 4) {
            screenData[i] = 0;      // R
            screenData[i + 1] = 0;  // G
            screenData[i + 2] = 64; // B
            screenData[i + 3] = 255; // A
        }

        this.screenTexture = new THREE.DataTexture(screenData, 320, 240, THREE.RGBAFormat);
        this.screenTexture.flipY = true;
        this.screenTexture.magFilter = THREE.NearestFilter; // Pixelated look
        this.screenTexture.minFilter = THREE.NearestFilter;

        // Screen
        const screenGeometry = new THREE.PlaneGeometry(0.4, 0.3);
        const screenMaterial = new THREE.MeshBasicMaterial({ 
            map: this.screenTexture,
            emissive: new THREE.Color(0x002244)
        });
        const screen = new THREE.Mesh(screenGeometry, screenMaterial);
        screen.position.set(0, 1.3, 0.325);
        this.arcadeMachine.add(screen);

        // Control panel
        const controlGeometry = new THREE.BoxGeometry(0.7, 0.1, 0.3);
        const controlMaterial = new THREE.MeshLambertMaterial({ color: 0x666666 });
        const controlPanel = new THREE.Mesh(controlGeometry, controlMaterial);
        controlPanel.position.set(0, 1.0, 0.15);
        controlPanel.rotation.x = -0.2;
        this.arcadeMachine.add(controlPanel);

        // Joystick
        const stickGeometry = new THREE.CylinderGeometry(0.02, 0.02, 0.15);
        const stickMaterial = new THREE.MeshLambertMaterial({ color: 0xff0000 });
        const joystick = new THREE.Mesh(stickGeometry, stickMaterial);
        joystick.position.set(-0.15, 1.12, 0.05);
        this.arcadeMachine.add(joystick);

        // Buttons
        const buttonGeometry = new THREE.CylinderGeometry(0.03, 0.03, 0.02);
        const buttonMaterial = new THREE.MeshLambertMaterial({ color: 0x00ff00 });
        
        for (let i = 0; i < 6; i++) {
            const button = new THREE.Mesh(buttonGeometry, buttonMaterial);
            button.position.set(0.05 + (i % 3) * 0.08, 1.08, 0.05 + Math.floor(i / 3) * 0.08);
            this.arcadeMachine.add(button);
        }

        // Marquee
        const marqueeGeometry = new THREE.BoxGeometry(0.6, 0.15, 0.03);
        const marqueeMaterial = new THREE.MeshLambertMaterial({ color: 0x00ff88 });
        const marquee = new THREE.Mesh(marqueeGeometry, marqueeMaterial);
        marquee.position.set(0, 1.7, 0.3);
        this.arcadeMachine.add(marquee);

        // Add some neon glow
        const glowGeometry = new THREE.PlaneGeometry(0.65, 0.2);
        const glowMaterial = new THREE.MeshBasicMaterial({ 
            color: 0x00ff88, 
            transparent: true, 
            opacity: 0.3 
        });
        const glow = new THREE.Mesh(glowGeometry, glowMaterial);
        glow.position.set(0, 1.7, 0.32);
        this.arcadeMachine.add(glow);

        this.scene.add(this.arcadeMachine);
    }

    async initializeZGS() {
        try {
            console.log('🚀 Creating ZebratronCartridgeSystem...');
            this.system = new ZebratronCartridgeSystem();
            
            // Create a virtual canvas for ZGS (it won't be displayed)
            const virtualCanvas = document.createElement('canvas');
            virtualCanvas.width = 320;
            virtualCanvas.height = 240;
            
            console.log('🚀 Initializing system...');
            await this.system.initialize(virtualCanvas);
            console.log('✅ System initialized successfully!');
            this.updateStatus('System ready - Load a cartridge');
        } catch (error) {
            console.error('❌ Failed to initialize system:', error);
            this.updateStatus('Failed to initialize system');
        }
    }

    setupUI() {
        const loadHambertBtn = document.getElementById('loadHambertBtn');
        const loadZSynthBtn = document.getElementById('loadZSynthBtn');
        const startBtn = document.getElementById('startBtn');
        const stopBtn = document.getElementById('stopBtn');
        const resetBtn = document.getElementById('resetBtn');

        loadHambertBtn.addEventListener('click', () => this.loadHambert());
        loadZSynthBtn.addEventListener('click', () => this.loadZSynth());
        startBtn.addEventListener('click', () => this.start());
        stopBtn.addEventListener('click', () => this.stop());
        resetBtn.addEventListener('click', () => this.reset());
    }

    setupKeyboardListeners() {
        document.addEventListener('keydown', (event) => {
            const key = event.key.toLowerCase();
            
            // Game controls
            if (['arrowup', 'arrowdown', 'arrowleft', 'arrowright'].includes(event.key)) {
                this.handleGameInput();
                event.preventDefault();
            }
            
            // Z-Synth controls
            const synthKeys = 'zsxdcvgbhnjm';
            if (synthKeys.includes(key) && this.currentCartridge === 'zsynth' && this.system) {
                if (!this.activeKeys.has(key)) {
                    this.activeKeys.add(key);
                    this.system.handleZSynthKeyDown(key);
                }
                event.preventDefault();
            }
        });

        document.addEventListener('keyup', (event) => {
            const key = event.key.toLowerCase();
            
            // Game controls
            if (['arrowup', 'arrowdown', 'arrowleft', 'arrowright'].includes(event.key)) {
                this.handleGameInput();
                event.preventDefault();
            }
            
            // Z-Synth controls
            const synthKeys = 'zsxdcvgbhnjm';
            if (synthKeys.includes(key) && this.currentCartridge === 'zsynth' && this.system) {
                if (this.activeKeys.has(key)) {
                    this.activeKeys.delete(key);
                    this.system.handleZSynthKeyUp(key);
                }
                event.preventDefault();
            }
        });
    }

    handleGameInput() {
        if (!this.system || this.currentCartridge !== 'hambert') return;
        
        const keys = {
            up: false,
            down: false,
            left: false,
            right: false
        };
        
        // Check current key states
        if (event.key === 'ArrowUp') keys.up = event.type === 'keydown';
        if (event.key === 'ArrowDown') keys.down = event.type === 'keydown';
        if (event.key === 'ArrowLeft') keys.left = event.type === 'keydown';
        if (event.key === 'ArrowRight') keys.right = event.type === 'keydown';
        
        this.system.handleInput(keys.up, keys.down, keys.left, keys.right);
    }

    loadHambert() {
        if (!this.system) return;
        try {
            console.log('🐹 Loading Hambert cartridge...');
            const success = this.system.loadHambertCartridge();
            if (success) {
                this.currentCartridge = 'hambert';
                this.updateStatus('Hambert loaded - Use arrow keys');
            }
        } catch (error) {
            console.error('❌ Error loading Hambert:', error);
        }
    }

    loadZSynth() {
        if (!this.system) return;
        try {
            console.log('🎹 Loading Z-Synth cartridge...');
            const success = this.system.loadZSynthCartridge();
            if (success) {
                this.currentCartridge = 'zsynth';
                this.updateStatus('Z-Synth loaded - Use ZSXDCVGBHNJM keys');
            }
        } catch (error) {
            console.error('❌ Error loading Z-Synth:', error);
        }
    }

    async start() {
        if (this.isRunning || !this.system) return;
        try {
            await this.system.start();
            this.isRunning = true;
            this.updateStatus('Running - Enjoy the 3D arcade!');
        } catch (error) {
            console.error('❌ Failed to start system:', error);
        }
    }

    stop() {
        this.isRunning = false;
        if (this.system) {
            this.system.stop();
        }
        this.updateStatus('Stopped');
        this.activeKeys.clear();
    }

    reset() {
        if (this.system) {
            this.system.reset();
            this.updateStatus('Reset');
        }
    }

    animate() {
        requestAnimationFrame(() => this.animate());

        // Update ZGS
        if (this.isRunning && this.system) {
            try {
                const frameReady = this.system.stepFrame();
                if (frameReady) {
                    this.system.render();
                    
                    // Update 3D screen texture with ZGS output
                    const buffer = this.system.get_screen_buffer();
                    if (buffer && this.screenTexture) {
                        this.screenTexture.image.data = new Uint8Array(buffer);
                        this.screenTexture.needsUpdate = true;
                    }
                }
            } catch (error) {
                console.error('❌ Game loop error:', error);
            }
        }

        // Update 3D scene
        this.controls.update();
        this.renderer.render(this.scene, this.camera);
    }

    onWindowResize() {
        this.camera.aspect = window.innerWidth / window.innerHeight;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(window.innerWidth, window.innerHeight);
    }

    updateStatus(message) {
        const statusElement = document.getElementById('status');
        if (statusElement) {
            statusElement.textContent = message;
        }
    }
}

// Initialize the 3D arcade demo
const demo = new Arcade3DDemo();
console.log('🕹️ 3D Arcade Demo initialized!');