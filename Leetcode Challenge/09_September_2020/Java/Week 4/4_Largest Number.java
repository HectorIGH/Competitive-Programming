//Given a list of non negative integers, arrange them such that they form the largest number.
//
//Example 1:
//
//Input: [10,2]
//Output: "210"
//Example 2:
//
//Input: [3,30,34,5,9]
//Output: "9534330"
//Note: The result may be very large, so you need to return a string instead of an integer.

class Solution {
    
    private class CustomComparator implements Comparator<String> {
        @Override
        public int compare(String x, String y) {
            String order1 = x + y;
            String order2 = y + x;
           return order2.compareTo(order1);
        }
    }
    
    public String largestNumber(int[] nums) {
        List<String> s = new ArrayList<>();
        for(int n : nums) {
            s.add(String.valueOf(n));
        }
        String ans = "";
        //Collections.sort(s, new CustomComparator());
        Collections.sort(s, (x, y) -> (y + x).compareTo(x + y));
        if(s.get(0).equals("0")) {
            return "0";
        } else {
            return String.join("", s);
        }
    }
}