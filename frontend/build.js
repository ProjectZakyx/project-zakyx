/**
 * ZAKYXBrowser Build Script
 * Kombiniert alle Module zu einer einzigen JavaScript-Datei
 */

const fs = require('fs');
const path = require('path');

// Build-Konfiguration
const BUILD_CONFIG = {
    inputDir: './frontend',
    outputFile: '../dist/app.js',
    modules: [
        'utils/utils.js',
        'modules/core.js',
        'modules/navigation.js',
        'modules/tabManager.js',
        'modules/bookmarkManager.js',
        'zakyxBrowser.js'
    ],
    banner: `/**
 * ZAKYXBrowser v1.0.0 - Modularer Web Browser
 * Generiert am: ${new Date().toISOString()}
 * 
 * Module:
 * - Core: Grundfunktionalität
 * - Navigation Manager: URL-Handling und Proxy
 * - Tab Manager: Tab-Verwaltung
 * - Bookmark Manager: Lesezeichen-System
 * - Utils: Helper-Funktionen
 */`
};

function readModuleFile(modulePath) {
    const fullPath = path.join(__dirname, modulePath);
    
    if (!fs.existsSync(fullPath)) {
        console.error(`❌ Module not found: ${modulePath}`);
        return '';
    }
    
    let content = fs.readFileSync(fullPath, 'utf8');
    
    // Entferne ES6 Import/Export Statements
    content = content.replace(/^import\s+.*from\s+.*$/gm, '// Import removed in build');
    content = content.replace(/^export\s+.*$/gm, '// Export removed in build');
    content = content.replace(/^export\s*\{[\s\S]*?\}\s*;?\s*$/gm, '// Export removed in build');
    
    // Füge Module-Kommentar hinzu
    const moduleComment = `\n// ===== MODULE: ${modulePath} =====\n`;
    const moduleEnd = `\n// ===== END MODULE: ${modulePath} =====\n`;
    
    return moduleComment + content + moduleEnd;
}

function buildZAKYXBrowser() {
    console.log('🔨 Building ZAKYXBrowser...');
    console.log('📁 Input directory:', BUILD_CONFIG.inputDir);
    console.log('📄 Output file:', BUILD_CONFIG.outputFile);
    
    let combinedContent = BUILD_CONFIG.banner + '\n\n';
    
    // Kombiniere alle Module
    BUILD_CONFIG.modules.forEach((module, index) => {
        console.log(`📦 Processing module ${index + 1}/${BUILD_CONFIG.modules.length}: ${module}`);
        
        const moduleContent = readModuleFile(module);
        if (moduleContent) {
            combinedContent += moduleContent + '\n';
        }
    });
    
    // Erstelle dist Verzeichnis falls es nicht existiert
    const distDir = path.dirname(BUILD_CONFIG.outputFile);
    if (!fs.existsSync(distDir)) {
        fs.mkdirSync(distDir, { recursive: true });
    }
    
    // Schreibe kombinierte Datei
    fs.writeFileSync(BUILD_CONFIG.outputFile, combinedContent);
    
    const stats = fs.statSync(BUILD_CONFIG.outputFile);
    const fileSizeKB = Math.round(stats.size / 1024);
    
    console.log(`✅ Build complete!`);
    console.log(`📄 Output: ${BUILD_CONFIG.outputFile}`);
    console.log(`📊 Size: ${fileSizeKB} KB`);
    console.log(`🔢 Lines: ${combinedContent.split('\n').length}`);
    
    return true;
}

// Alternative: Minimierte Version
function buildMinified() {
    console.log('🗜️ Building minified version...');
    
    const sourceContent = fs.readFileSync(BUILD_CONFIG.outputFile, 'utf8');
    
    // Einfache Minifizierung (entferne Kommentare und extra Leerzeichen)
    let minified = sourceContent
        // Entferne einzeilige Kommentare
        .replace(/\/\/.*$/gm, '')
        // Entferne mehrzeilige Kommentare  
        .replace(/\/\*[\s\S]*?\*\//g, '')
        // Entferne überschüssige Leerzeichen
        .replace(/\s+/g, ' ')
        // Entferne Leerzeichen um Operatoren
        .replace(/\s*([{}();,:])\s*/g, '$1')
        .trim();
    
    const minPath = BUILD_CONFIG.outputFile.replace('.js', '.min.js');
    fs.writeFileSync(minPath, minified);
    
    const minStats = fs.statSync(minPath);
    const minSizeKB = Math.round(minStats.size / 1024);
    
    console.log(`✅ Minified build complete!`);
    console.log(`📄 Output: ${minPath}`);
    console.log(`📊 Size: ${minSizeKB} KB`);
    
    return true;
}

// Development Watch-Modus
function watchMode() {
    console.log('👀 Starting watch mode...');
    
    BUILD_CONFIG.modules.forEach(module => {
        const fullPath = path.join(__dirname, '..', 'frontend', module);
        
        if (fs.existsSync(fullPath)) {
            fs.watchFile(fullPath, (curr, prev) => {
                            console.log(`🔄 File changed: ${module}`);
            buildZAKYXBrowser();
            });
        }
    });
    
    console.log('✅ Watching for changes...');
}

// Haupt-Build-Funktion
function main() {
    const args = process.argv.slice(2);
    
    if (args.includes('--watch')) {
        buildZAKYXBrowser();
        watchMode();
    } else if (args.includes('--minify')) {
        buildZAKYXBrowser();
        buildMinified();
    } else {
        buildZAKYXBrowser();
    }
}

// Starte Build wenn direkt ausgeführt
if (require.main === module) {
    main();
}

module.exports = {
    buildZAKYXBrowser,
    buildMinified,
    watchMode
}; 