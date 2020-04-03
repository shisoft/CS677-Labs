import org.json.simple.JSONArray;
import org.json.simple.JSONObject;

import java.io.FileWriter;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class makeDB {
        public static void main(String[] args) throws IOException {

            List<List<String>> rows = Arrays.asList(
                    Arrays.asList("How to get a good grade in 677 in 20 minutes a day", "1", "10", "19.9","distributed systems"),
                    Arrays.asList("RPCs for Dummies", "2", "10", "14.9","distributed systems"),
                    Arrays.asList("Xen and the Art of Surviving Graduate School", "3", "10", "29.9","graduate school"),
                    Arrays.asList("Cooking for the Impatient Graduate Student", "4", "10", "39.9","graduate school")
                    );

            FileWriter csvWriter = new FileWriter("inventory.csv");
            csvWriter.append("name");
            csvWriter.append(",");
            csvWriter.append("id");
            csvWriter.append(",");
            csvWriter.append("stock");
            csvWriter.append(",");
            csvWriter.append("price");
            csvWriter.append(",");
            csvWriter.append("topic");
            csvWriter.append("\n");

            for (List<String> rowData : rows) {
                csvWriter.append(String.join(",", rowData));
                csvWriter.append("\n");
            }

            csvWriter.flush();
            csvWriter.close();
        }
//    public static void main(String[] args) {
//        JSONObject jo = new JSONObject();
//        jo.put("name", "How to get a good grade in 677 in 20 minutes a day");
//        jo.put("id", "1");
//        jo.put("stock", "10");
//        jo.put("price", "20");
//        jo.put("topic", "distributed systems");
//
//        JSONObject jo1 = new JSONObject();
//        jo1.put("name", "RPCs for Dummies");
//        jo1.put("id", "2");
//        jo1.put("stock", "10");
//        jo1.put("price", "24");
//        jo1.put("topic", "distributed systems");
//
//        JSONObject jo2 = new JSONObject();
//        jo2.put("name", "Xen and the Art of Surviving Graduate School");
//        jo2.put("id", "3");
//        jo2.put("stock", "10");
//        jo2.put("price", "30");
//        jo2.put("topic", "graduate school");
//
//        JSONObject jo3 = new JSONObject();
//        jo3.put("name", "Cooking for the Impatient Graduate Student");
//        jo3.put("id", "4");
//        jo3.put("stock", "10");
//        jo3.put("price", "49");
//        jo3.put("topic", "graduate school");
//
//
//
//
//        JSONArray ja = new JSONArray();
//        ja.add(jo);
//        ja.add(jo1);
//        ja.add(jo2);
//        ja.add(jo3);
//
//        JSONObject bo = new JSONObject();
//
//        bo.put("books", ja);
//
//        try (FileWriter file = new FileWriter("inventory.json")) {
//
//            file.write(bo.toJSONString());
//            file.flush();
//
//        } catch (IOException e) {
//            e.printStackTrace();
//        }
//
//    }
}
