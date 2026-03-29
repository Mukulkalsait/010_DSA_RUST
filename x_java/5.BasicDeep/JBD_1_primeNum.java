import java.util.*;

public class JBD_1_primeNum {

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please input a number to check PRIME:");
    int checkNum = sc.nextInt();
    Boolean prime = true;
    if (checkNum == 2) {
      System.out.println("The number " + checkNum + " is Prime number!!!");
      sc.close();
      return;
    } else {
      for (int i = 2; i <= Math.sqrt(checkNum); i++) {
        if (checkNum % i == 0) {
          prime = false;
          System.out.println(checkNum + " is devisable by " + i + " Hence:");
          break;
        }
      }

      // System.out.println(prime);
      if (prime == true) {
        System.out.println("The number " + checkNum + " is Prime number!!!");
      } else {
        System.out.println("The number " + checkNum + " is NOT a Prime number!!!");
      }
    }
    sc.close();
  }
}

// NOTE:
// 1. FACTORS allways repeats.
// eg. 12 =
// ------
// 1 * 12
// 2 * 6
// 3 * 4
// ------
// 4 * 3
// 6 * 2
// 12 * 1
// ======
//
// FIX: you can see after half way they got repeated that half way is not the
// half of numbers
// but it can be defined as =================
// 2 divesers who are closest of each other or equal.
// eg
// 1. 3 * 4 in 12
// 2. 4 * 4 in 16
// 3. 4 * 5 in 20
//
// NOTE:
// in 1 and and 3 we can see that the numbrs are close so thats the last number
// where we should stop the loop should be 3 or 4 in 12 and 4 or 5 in 20. but in
// case of 16 we can see that the 4 * 4 got teh number but unlike 16 or
// other square numbers we cannot define theri relation so we use underroot
//
// TODO:
// . ____ ... ____
// \/ num * \/ num
//
// NOTE: with this we can narrow down serch to extream level just look at the
// 2nd example
// to check the devidiblity of 20 we only checked till 4 from 5th it will be
// repeted.
//
