// JavaScript - Iterator Invalidation Bug Example
// This demonstrates what Rust prevents at compile time!

console.log("=== ITERATOR INVALIDATION BUG ===\n");

// ══════════════════════════════════════════════════════════════
// EXAMPLE 1: Adding element while iterating (JS engine tries to help)
// ══════════════════════════════════════════════════════

console.log("--- Example 1: Push while iterating ---");
let nums = [1, 2, 3];
console.log("Before:", nums);

// In modern JavaScript, this might actually work 
// because the iterator detects the change
try {
    for (const n of nums) {
        console.log(`  Iterator at: ${n}`);
        if (n === 1) {
            console.log("  → Pushing 10...");
            nums.push(10);
            console.log("  → nums is now:", nums);
        }
    }
} catch (e) {
    console.log("ERROR:", e.message);
}

console.log("\n--- Example 2: Using index - same problem ---");
// Using traditional for-loop with index - NO protection!
nums = [1, 2, 3];
console.log("Before:", nums);

for (let i = 0; i < nums.length; i++) {
    console.log(`  Index ${i}, value: ${nums[i]}`);
    if (nums[i] === 1) {
        console.log("  → Pushing 10...");
        nums.push(10);
        console.log("  → After push, nums:", nums);
        // Now i=1 points to 2, but array grew!
        // This can cause infinite loop!
    }
}
console.log("Result:", nums);
console.log("(Notice: it added 10 twice - because loop kept going!)");

// ══════════════════════════════════════════════════════
// EXAMPLE 3: Deleting while iterating - MORE DANGEROUS
// ══════════════════════════════════════════════════════

console.log("\n--- Example 3: Delete while iterating ---");
let items = [1, 2, 3, 4, 5];
console.log("Before:", items);

for (const item of items) {
    console.log(`  Processing: ${item}`);
    if (item === 2) {
        // Remove element - causes re-indexing!
        const idx = items.indexOf(2);
        items.splice(idx, 1);
        console.log("  → After delete:", items);
    }
}
console.log("Final:", items);
console.log("(Notice: 3 and 4 were skipped!)");

// ══════════════════════════════════════════════════════
// EXAMPLE 4: What SHOULD happen (Rust model)
// ══════════════════════════════════════════════════════

console.log("\n--- CORRECT WAY: Collect first, then modify ---");
nums = [1, 2, 3];

// Option A: Collect indices first
let indices = [...nums.keys()]; // [0, 1, 2]
console.log("Indices to process:", indices);

for (const i of indices) {
    console.log(`  Processing index ${i}: ${nums[i]}`);
    if (nums[i] === 1) {
        nums.push(10);
    }
}
console.log("Final:", nums);

console.log("\n=== This is what Rust prevents at compile time! ===");