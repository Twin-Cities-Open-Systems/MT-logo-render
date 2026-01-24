# 🎨 MT-Logo-Render Usage Examples & Demo

This document provides comprehensive examples of how to use MT-Logo-Render to create beautiful logos and visual elements for the Market Thesis ecosystem.

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Basic Shapes](#basic-shapes)
- [Color Patterns](#color-patterns)
- [Text Rendering](#text-rendering)
- [Advanced Recipes](#advanced-recipes)
- [Batch Processing](#batch-processing)
- [Integration Examples](#integration-examples)

## 🚀 Quick Start

### Install MT-Logo-Render

```bash
# Download the latest release for your platform
curl -L https://github.com/spencerbutler/MT-logo-render/releases/latest/download/logo-render-linux-x64 -o logo-render
chmod +x logo-render
```

### Basic Usage

```bash
# Render a simple circle logo
echo '{"shape": "circle", "size": "256x256", "fill": "#FF6B35"}' | ./logo-render render -o logo.png

# Render with ANSI output to terminal
echo '{"shape": "circle", "size": "32x32", "fill": "#FF6B35"}' | ./logo-render render --format ansi
```

## 🔵 Basic Shapes

### Circle Logos

```bash
# Basic circle
echo '{
  "shape": "circle",
  "size": "256x256",
  "fill": "#FF6B35",
  "border": {
    "width": 4,
    "color": "#000000"
  }
}' | ./logo-render render -o circle_basic.png
```

![Circle Basic](https://via.placeholder.com/256x256/FF6B35/FFFFFF?text=Circle)

### Rectangle/Square Logos

```bash
# Square with rounded corners
echo '{
  "shape": "rectangle",
  "size": "256x256",
  "fill": "#4ECDC4",
  "border": {
    "width": 3,
    "color": "#45B7AA"
  },
  "corner_radius": 32
}' | ./logo-render render -o square_rounded.png
```

![Square Rounded](https://via.placeholder.com/256x256/4ECDC4/FFFFFF?text=Square)

### Triangle Logos

```bash
# Equilateral triangle
echo '{
  "shape": "triangle",
  "size": "256x256",
  "fill": "#A8E6CF",
  "border": {
    "width": 2,
    "color": "#96D4BC"
  }
}' | ./logo-render render -o triangle.png
```

![Triangle](https://via.placeholder.com/256x256/A8E6CF/000000?text=Triangle)

## 🎨 Color Patterns

### Gradient Fills

```bash
# Linear gradient
echo '{
  "shape": "circle",
  "size": "256x256",
  "fill": {
    "type": "linear_gradient",
    "colors": ["#FF6B35", "#F7931E"],
    "direction": "diagonal"
  }
}' | ./logo-render render -o gradient_linear.png
```

![Linear Gradient](https://via.placeholder.com/256x256/FF6B35/F7931E?text=Gradient)

### Pattern Fills

```bash
# Striped pattern
echo '{
  "shape": "rectangle",
  "size": "256x256",
  "fill": {
    "type": "stripe",
    "colors": ["#FF6B35", "#FFFFFF"],
    "width": 16
  }
}' | ./logo-render render -o stripes.png
```

![Stripes](https://via.placeholder.com/256x256/FF6B35/FFFFFF?text=Stripes)

### Checkerboard Pattern

```bash
# Checkerboard fill
echo '{
  "shape": "square",
  "size": "256x256",
  "fill": {
    "type": "checkerboard",
    "colors": ["#FF6B35", "#4ECDC4"],
    "size": 32
  }
}' | ./logo-render render -o checkerboard.png
```

![Checkerboard](https://via.placeholder.com/256x256/FF6B35/4ECDC4?text=Checker)

## 📝 Text Rendering

### Basic Text

```bash
# Simple text logo
echo '{
  "shape": "rectangle",
  "size": "512x128",
  "fill": "#2C3E50",
  "text": {
    "content": "MARKET THESIS",
    "font_size": 48,
    "color": "#ECF0F1",
    "align": "center"
  }
}' | ./logo-render render -o text_basic.png
```

![Basic Text](https://via.placeholder.com/512x128/2C3E50/ECF0F1?text=MARKET+THESIS)

### Text with Background Pattern

```bash
# Text over striped background
echo '{
  "shape": "rectangle",
  "size": "512x128",
  "fill": {
    "type": "stripe",
    "colors": ["#3498DB", "#2980B9"],
    "width": 8
  },
  "text": {
    "content": "MT LOGO",
    "font_size": 52,
    "color": "#FFFFFF",
    "align": "center",
    "font_weight": "bold"
  }
}' | ./logo-render render -o text_striped.png
```

![Text Striped](https://via.placeholder.com/512x128/3498DB/FFFFFF?text=MT+LOGO)

## 🔧 Advanced Recipes

### Multi-Layer Logos

```bash
# Logo with multiple elements
echo '{
  "layers": [
    {
      "shape": "circle",
      "size": "200x200",
      "position": [28, 28],
      "fill": "#FF6B35"
    },
    {
      "shape": "triangle",
      "size": "100x100",
      "position": [78, 78],
      "fill": "#FFFFFF"
    },
    {
      "text": {
        "content": "MT",
        "font_size": 48,
        "color": "#2C3E50",
        "position": [128, 128],
        "align": "center"
      }
    }
  ],
  "size": "256x256",
  "background": "#F8F9FA"
}' | ./logo-render render -o multi_layer.png
```

![Multi Layer](https://via.placeholder.com/256x256/F8F9FA/000000?text=MT)

### Logo with Shadow Effect

```bash
# Logo with drop shadow
echo '{
  "layers": [
    {
      "shape": "circle",
      "size": "200x200",
      "position": [35, 35],
      "fill": "#000000",
      "opacity": 0.3
    },
    {
      "shape": "circle",
      "size": "200x200",
      "position": [25, 25],
      "fill": "#FF6B35",
      "border": {
        "width": 4,
        "color": "#E74C3C"
      }
    }
  ],
  "size": "256x256",
  "background": "#FFFFFF"
}' | ./logo-render render -o shadow_effect.png
```

![Shadow Effect](https://via.placeholder.com/256x256/FFFFFF/FF6B35?text=Shadow)

## 📦 Batch Processing

### Process Multiple Recipes

```bash
# Create a batch file with multiple recipes
cat > batch_recipes.json << 'EOF'
[
  {
    "id": "logo_primary",
    "recipe": {
      "shape": "circle",
      "size": "256x256",
      "fill": "#FF6B35"
    }
  },
  {
    "id": "logo_secondary",
    "recipe": {
      "shape": "square",
      "size": "256x256",
      "fill": "#4ECDC4"
    }
  },
  {
    "id": "logo_text",
    "recipe": {
      "shape": "rectangle",
      "size": "512x128",
      "fill": "#2C3E50",
      "text": {
        "content": "MARKET THESIS",
        "color": "#ECF0F1"
      }
    }
  }
]
EOF

# Process batch
./logo-render batch batch_recipes.json --output-dir ./logos/
```

### Automated Logo Generation

```bash
# Generate logos for different themes
declare -a themes=(
  '{"name": "sunset", "primary": "#FF6B35", "secondary": "#F7931E"}'
  '{"name": "ocean", "primary": "#3498DB", "secondary": "#2980B9"}'
  '{"name": "forest", "primary": "#27AE60", "secondary": "#2ECC71"}'
)

for theme in "${themes[@]}"; do
  name=$(echo $theme | jq -r .name)
  primary=$(echo $theme | jq -r .primary)
  secondary=$(echo $theme | jq -r .secondary)

  echo "{
    \"shape\": \"circle\",
    \"size\": \"256x256\",
    \"fill\": {
      \"type\": \"linear_gradient\",
      \"colors\": [\"$primary\", \"$secondary\"],
      \"direction\": \"diagonal\"
    }
  }" | ./logo-render render -o "logo_${name}.png"
done
```

## 🔗 Integration Examples

### Web Integration

```html
<!-- HTML integration example -->
<!DOCTYPE html>
<html>
<head>
  <title>Market Thesis Dashboard</title>
  <style>
    .mt-logo {
      width: 64px;
      height: 64px;
      border-radius: 50%;
      background: linear-gradient(135deg, #FF6B35, #F7931E);
      display: inline-block;
    }
  </style>
</head>
<body>
  <div class="mt-logo"></div>
  <h1>Market Thesis Platform</h1>

  <!-- Or use generated images -->
  <img src="logo_mt.png" alt="Market Thesis Logo" width="128" height="128">
</body>
</html>
```

### API Integration (Node.js)

```javascript
const { execSync } = require('child_process');
const fs = require('fs');

class MTLogoGenerator {
  constructor(binaryPath = './logo-render') {
    this.binaryPath = binaryPath;
  }

  async generateLogo(recipe, outputPath) {
    const recipeJson = JSON.stringify(recipe);

    try {
      execSync(`echo '${recipeJson}' | ${this.binaryPath} render -o ${outputPath}`);
      return { success: true, path: outputPath };
    } catch (error) {
      return { success: false, error: error.message };
    }
  }

  async generateBrandKit(theme) {
    const recipes = [
      {
        shape: 'circle',
        size: '256x256',
        fill: theme.primary,
        id: 'logo_primary'
      },
      {
        shape: 'circle',
        size: '64x64',
        fill: theme.primary,
        id: 'logo_small'
      },
      {
        shape: 'rectangle',
        size: '1200x630',
        fill: theme.secondary,
        text: {
          content: 'Market Thesis',
          color: '#FFFFFF',
          font_size: 72
        },
        id: 'banner'
      }
    ];

    const results = [];
    for (const recipe of recipes) {
      const outputPath = `brand_${recipe.id}.png`;
      const result = await this.generateLogo(recipe, outputPath);
      results.push({ ...recipe, ...result });
    }

    return results;
  }
}

// Usage
const generator = new MTLogoGenerator();

const sunsetTheme = {
  primary: '#FF6B35',
  secondary: '#F7931E'
};

generator.generateBrandKit(sunsetTheme)
  .then(results => console.log('Brand kit generated:', results))
  .catch(error => console.error('Error:', error));
```

### CLI Automation Script

```bash
#!/bin/bash
# MT Logo Generation Script

set -e

# Configuration
LOGO_RENDER="./logo-render"
OUTPUT_DIR="./generated-logos"
THEMES_FILE="themes.json"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Generate logos for each theme
while IFS= read -r theme_json; do
  theme_name=$(echo "$theme_json" | jq -r '.name')
  primary_color=$(echo "$theme_json" | jq -r '.primary')
  secondary_color=$(echo "$theme_json" | jq -r '.secondary')

  echo "Generating logos for theme: $theme_name"

  # Primary logo
  echo "{
    \"shape\": \"circle\",
    \"size\": \"256x256\",
    \"fill\": \"$primary_color\"
  }" | "$LOGO_RENDER" render -o "$OUTPUT_DIR/logo_${theme_name}_primary.png"

  # Secondary logo
  echo "{
    \"shape\": \"circle\",
    \"size\": \"256x256\",
    \"fill\": {
      \"type\": \"linear_gradient\",
      \"colors\": [\"$primary_color\", \"$secondary_color\"]
    }
  }" | "$LOGO_RENDER" render -o "$OUTPUT_DIR/logo_${theme_name}_gradient.png"

  echo "✓ Generated logos for $theme_name"
done < <(jq -c '.[]' "$THEMES_FILE")

echo "🎉 Logo generation complete! Check $OUTPUT_DIR for results."
```

## 🎯 Best Practices

### Recipe Optimization

1. **Use Consistent Sizing**: Standardize logo sizes (256x256 for icons, 512x512 for main logos)
1. **Color Harmony**: Use complementary colors from the Market Thesis palette
1. **Scalable Formats**: Generate multiple sizes for different use cases
1. **Cache Effectively**: Use the built-in caching for repeated renders

### Performance Tips

1. **Batch Processing**: Process multiple logos in batches for efficiency
1. **Cache Reuse**: Leverage the fingerprint-based caching system
1. **Parallel Generation**: Use multiple processes for large batches
1. **Resource Limits**: Monitor memory usage for large renders

### Quality Assurance

1. **Visual Inspection**: Always review generated logos
1. **Cross-Platform Testing**: Test on different operating systems
1. **Color Accuracy**: Verify colors match design specifications
1. **Scalability**: Ensure logos look good at different sizes

## 📞 Support & Resources

- **Documentation**: [MT-Logo-Render Docs](./)
- **Issues**: [GitHub Issues](https://github.com/spencerbutler/MT-logo-render/issues)
- **Discussions**: [GitHub Discussions](https://github.com/spencerbutler/MT-logo-render/discussions)

______________________________________________________________________

*Generated with ❤️ by the Market Thesis team*
