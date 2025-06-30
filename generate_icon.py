#!/usr/bin/env python3
"""
Script to create the QR Forge icon based on the design provided.
This recreates the QR code + anvil logo as PNG.
"""

try:
    from PIL import Image, ImageDraw, ImageFont
    PIL_AVAILABLE = True
except ImportError:
    PIL_AVAILABLE = False

def create_qr_forge_icon():
    if not PIL_AVAILABLE:
        print("❌ PIL not available. Please install with: pip install Pillow")
        return False
    
    # Create 1024x1024 image with light background
    size = 1024
    img = Image.new('RGBA', (size, size), (240, 240, 240, 255))
    draw = ImageDraw.Draw(img)
    
    # QR Code section (top part)
    qr_margin = 150
    qr_size = 300
    qr_x = (size - qr_size) // 2
    qr_y = qr_margin
    
    # QR code cell size
    cell_size = qr_size // 9
    
    # Draw QR finder patterns (corners)
    def draw_finder_pattern(x_offset, y_offset):
        # Outer black square
        draw.rectangle([
            qr_x + x_offset * cell_size,
            qr_y + y_offset * cell_size,
            qr_x + (x_offset + 3) * cell_size,
            qr_y + (y_offset + 3) * cell_size
        ], fill=(0, 0, 0, 255))
        
        # Inner white square
        draw.rectangle([
            qr_x + (x_offset + 0.5) * cell_size,
            qr_y + (y_offset + 0.5) * cell_size,
            qr_x + (x_offset + 2.5) * cell_size,
            qr_y + (y_offset + 2.5) * cell_size
        ], fill=(240, 240, 240, 255))
        
        # Center black square
        draw.rectangle([
            qr_x + (x_offset + 1) * cell_size,
            qr_y + (y_offset + 1) * cell_size,
            qr_x + (x_offset + 2) * cell_size,
            qr_y + (y_offset + 2) * cell_size
        ], fill=(0, 0, 0, 255))
    
    # Draw the three finder patterns
    draw_finder_pattern(0, 0)      # Top-left
    draw_finder_pattern(6, 0)      # Top-right  
    draw_finder_pattern(0, 6)      # Bottom-left
    
    # Add some data modules to make it look like a real QR code
    data_modules = [
        (3.5, 3.5), (4.5, 3.5), (5.5, 3.5),
        (3.5, 4.5), (5.5, 4.5),
        (3.5, 5.5), (4.5, 5.5)
    ]
    
    for x_pos, y_pos in data_modules:
        draw.rectangle([
            qr_x + x_pos * cell_size,
            qr_y + y_pos * cell_size,
            qr_x + (x_pos + 1) * cell_size,
            qr_y + (y_pos + 1) * cell_size
        ], fill=(0, 0, 0, 255))
    
    # Anvil section (bottom part)
    anvil_y = qr_y + qr_size + 80
    anvil_width = 450
    anvil_x = (size - anvil_width) // 2
    
    # Main anvil body
    body_height = 100
    body_radius = 20
    draw.rounded_rectangle([
        anvil_x,
        anvil_y,
        anvil_x + anvil_width * 0.7,  # Main body is 70% of total width
        anvil_y + body_height
    ], radius=body_radius, fill=(0, 0, 0, 255))
    
    # Anvil horn (pointed extension)
    horn_start_x = anvil_x + anvil_width * 0.7
    horn_points = [
        (horn_start_x, anvil_y + body_height * 0.2),
        (anvil_x + anvil_width, anvil_y + body_height * 0.5),
        (horn_start_x, anvil_y + body_height * 0.8)
    ]
    draw.polygon(horn_points, fill=(0, 0, 0, 255))
    
    # Anvil legs/base
    leg_width = 50
    leg_height = 80
    leg_y = anvil_y + body_height
    
    # Left leg
    draw.rectangle([
        anvil_x + 60,
        leg_y,
        anvil_x + 60 + leg_width,
        leg_y + leg_height
    ], fill=(0, 0, 0, 255))
    
    # Right leg
    draw.rectangle([
        anvil_x + anvil_width * 0.7 - 110,
        leg_y,
        anvil_x + anvil_width * 0.7 - 110 + leg_width,
        leg_y + leg_height
    ], fill=(0, 0, 0, 255))
    
    # Text "QR FORGE" at the bottom
    text_y = anvil_y + body_height + leg_height + 60
    
    try:
        # Try to use a bold font
        font_size = 72
        try:
            font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", font_size)
        except:
            try:
                font = ImageFont.truetype("arial.ttf", font_size)
            except:
                font = ImageFont.load_default()
        
        text = "QR FORGE"
        # Get text bounding box
        bbox = draw.textbbox((0, 0), text, font=font)
        text_width = bbox[2] - bbox[0]
        text_x = (size - text_width) // 2
        
        draw.text((text_x, text_y), text, fill=(0, 0, 0, 255), font=font)
    except:
        # Fallback without custom font
        text = "QR FORGE"
        # Approximate text positioning
        text_x = size // 2 - 120
        draw.text((text_x, text_y), text, fill=(0, 0, 0, 255))
    
    # Save the image
    img.save("image.png", "PNG")
    print("✅ QR Forge icon created successfully as image.png")
    return True

if __name__ == "__main__":
    if create_qr_forge_icon():
        print("🎉 Icon is ready! You can now run ./create_icon.sh to create the app icon.")
    else:
        print("❌ Failed to create icon. Please install Pillow: pip install Pillow")
