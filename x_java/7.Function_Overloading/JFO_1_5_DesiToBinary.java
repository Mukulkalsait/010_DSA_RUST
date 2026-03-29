
import java.util.*;

public class JFO_1_5_DesiToBinary {

  public static int converter(int num) {
    int res = 0;
    int multi = 1;

    do {
      int a = (num % 2) * multi;
      res = res + a;
      num = num / 2;
      multi *= 10; // TODO: dignoisis
                   // System.out.println(res);
    } while (num > 0);
    return res;
  }

  public static int converterModern(int num) {

    int pow = 0;
    int binaryNum = 0;
    while (num > 0) {
      binaryNum = binaryNum + (int) (num % 2 * Math.pow(10, pow));
      pow++;
      num = num / 2;
    }
    return binaryNum;
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("Inter the Decimal number for binary convertion:");
    int decimal = sc.nextInt();

    // GREEN: USE EITHER ONE OF THE BILLOW:
    // int binary = converter(decimal); //---------------1.
    int binary = converterModern(decimal); // ---------------2.

    //
    System.out.println("The binary convertion of " + binary + " is :" + decimal);
    sc.close();
  }
}
