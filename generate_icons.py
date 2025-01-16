from PIL import Image, ImageDraw

def create_icon(size):
    # Create a new image with a white background
    image = Image.new('RGBA', (size, size), (255, 255, 255, 0))
    draw = ImageDraw.Draw(image)
    
    # Draw a rounded rectangle
    padding = size // 8
    rect_size = size - (2 * padding)
    draw.rounded_rectangle(
        [(padding, padding), (size - padding, size - padding)],
        radius=size//8,
        fill=(52, 152, 219)  # Blue color
    )
    
    # Draw an arrow
    arrow_width = rect_size // 3
    arrow_height = rect_size // 2
    center_x = size // 2
    center_y = size // 2
    
    # Arrow points
    points = [
        (center_x - arrow_width//2, center_y + arrow_height//4),  # Bottom left
        (center_x + arrow_width//2, center_y + arrow_height//4),  # Bottom right
        (center_x, center_y - arrow_height//4)   # Top center
    ]
    
    draw.polygon(points, fill=(255, 255, 255))  # White arrow
    
    return image

# Generate icons in different sizes
sizes = {
    "32x32": 32,
    "128x128": 128,
    "128x128@2x": 256
}

for name, size in sizes.items():
    icon = create_icon(size)
    icon.save(f"icon/{name}.png")
