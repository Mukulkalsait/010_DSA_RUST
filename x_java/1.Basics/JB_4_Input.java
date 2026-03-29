
/**
 * JB_4_Input
 */

//import scanner from jdk
import java.util.*;

public class JB_4_Input {
  public static void main(String[] args) {
    // create scanner var as sc
    Scanner sc = new Scanner(System.in);

    // 1
    System.out.println("sc.next() Give me a line and i will return only 1st word.");
    String input = sc.next();
    // System.out.println("______");
    System.out.println(input);
    // System.out.println(input);
    // System.out.println(input);

    // 2
    System.out.println("sc.nextLine() Give me full para and i will return whole para.");
    String input2 = sc.nextLine();
    System.out.println(input2);
    // 3
    System.out.println("sc.nextInt() Give me a number and i will return its 300% .");
    int num = sc.nextInt();
    System.out.print(num * 3);

    // 4
    System.out.println("sc.nextFloat() i will return take floatpoiint as input.");
    float price = sc.nextFloat();
    System.out.println(price);

    // 5
    // nextByte()
    // //
    // next()
    // nextLine()
    // //
    // nextBoolean()
    // ..
    // nextShort()
    // nextInt()
    // nextLong()
    // ..
    // nextFolat()
    // nextDouble()
    sc.close();
  }
}

/*
 * TODO:
 * 1.......sc.next() will take only 1 input till the space like ony "TONY" in
 * "tony stark" will be taken as input
 *
 *
 * TODO:
 * 2....... sc>nextLint() will take whole line or even WHOLE PARAGRAPH as input.
 * 
 */
