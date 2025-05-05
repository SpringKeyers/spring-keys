#!/usr/bin/env python3
"""
Extract quotes from the original Rust source file and convert them to JSON files.

This script parses the original quotes.rs file and extracts the hardcoded quotes,
then saves them to separate JSON files by category.
"""

import os
import json
import re
from pathlib import Path

# Create necessary directories
os.makedirs("quotes/categories", exist_ok=True)

# Category mapping
categories = {
    "Proverbs": "proverbs.json",
    "TongueTwisters": "tongue_twisters.json",
    "Literature": "literature.json",
    "Programming": "programming.json",
    "Humor": "humor.json",
    "Multilingual": "multilingual.json",
    "Typewriters": "typewriters.json",
}

# Initialize empty arrays for each category
quotes_by_category = {cat: [] for cat in categories.keys()}

# Regular expression to extract quote information
quote_pattern = re.compile(
    r'Quote\s*\{\s*'
    r'text:\s*"([^"]*)".to_string\(\),\s*'
    r'source:\s*"([^"]*)".to_string\(\),\s*'
    r'difficulty:\s*QuoteDifficulty::(\w+),\s*'
    r'category:\s*QuoteCategory::(\w+),\s*'
    r'origin:\s*"([^"]*)".to_string\(\),\s*'
    r'\}'
)

def extract_quotes_from_file(file_path):
    """Extract quotes from the Rust source file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    print(f"Parsing quotes from {file_path}...")
    matches = quote_pattern.findall(content)
    print(f"Found {len(matches)} quotes")
    
    for match in matches:
        text, source, difficulty, category, origin = match
        
        # Create quote object
        quote = {
            "text": text,
            "source": source,
            "difficulty": difficulty,
            "category": category,
            "origin": origin
        }
        
        # Add to appropriate category
        if category in quotes_by_category:
            quotes_by_category[category].append(quote)
        else:
            print(f"Unknown category: {category}")

def save_quotes_to_json():
    """Save extracted quotes to JSON files by category."""
    for category, filename in categories.items():
        quotes = quotes_by_category[category]
        if not quotes:
            print(f"No quotes found for category {category}, skipping")
            continue
        
        filepath = Path(f"quotes/categories/{filename}")
        
        # Merge with existing quotes if the file exists
        if filepath.exists():
            with open(filepath, 'r') as f:
                existing_quotes = json.load(f)
            
            # Check for duplicates
            existing_texts = {q["text"] for q in existing_quotes}
            new_quotes = [q for q in quotes if q["text"] not in existing_texts]
            
            if not new_quotes:
                print(f"No new quotes to add for {category}")
                continue
            
            all_quotes = existing_quotes + new_quotes
            print(f"Adding {len(new_quotes)} new quotes to {len(existing_quotes)} existing quotes for {category}")
        else:
            all_quotes = quotes
            print(f"Creating new file with {len(quotes)} quotes for {category}")
        
        # Save to file
        with open(filepath, 'w') as f:
            json.dump(all_quotes, f, indent=2)
        
        print(f"Saved {len(all_quotes)} quotes to {filepath}")

if __name__ == "__main__":
    # Extract quotes from the Rust source file
    extract_quotes_from_file("src/quotes.rs")
    
    # Save to JSON files
    save_quotes_to_json()
    
    print("Quote extraction complete!") 