<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Responsive Hover Table                 _41d0a1</name>
   <tag></tag>
   <elementGuidId>8f039427-0bc4-4f1f-a60b-af9356b4e22e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Information'])[1]/following::div[1]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.content-wrapper</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>512fdfde-c3df-4769-bfd8-d045f5a84d99</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>content-wrapper</value>
      <webElementGuid>76c68574-467f-45f1-bfcf-d0bbcdcc0d60</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                
                
                
                    
    
        Responsive Hover Table
        
            
                
                    
                
                
                
                    
                
            
        
    
    
    
        
            
                    Username
                    Nama
                    Level
                    LastLogin
                    LastIP
                    Token
                    Tools
                
            
            
                
operator@oti.com
OperatorApliaksi
2
2023-05-09 14:03:50
182.4.103.211
93de4ce4fb4c9318f0c5baca0cd47635f7f026fbbc6b0b0939020ff4c1191b50456a71014ba16481744efaabea513b21015256a5035a9ad5990a3bd4f07fa3edrF_fPs56j7RoOJmdw7YiEdCCtvPzbCUwCva1hZtZUw8eUVAnM2DsAIiPtkeHztMy





muhammadyasin@oti.com
Muhammad Yasin
2
2023-05-09 13:38:42







Marvin@oti.com
Marvin
2
2023-05-09 13:40:26







wiuwiu@oti.com
wiuwiu
2
2023-05-09 13:48:14







sultonmukarobin@oti.com
Sulton Daud Ul Mukarobin
2
2023-05-09 13:48:21







muhammad.yasin@oti.com
Muhammad Yasin
2
2023-05-09 13:55:16







mugen@oti.com
MugenKun
2
2023-05-09 13:57:35







puterishofro@oti.com
Puteri Shofro
2
2023-05-09 13:59:33







yakobusarisarvanto@oti.com
Aris
2
2023-05-09 14:02:08







heleh@oti.com
heleh
2
2023-05-09 14:02:35






            
        
    
    
    
    12>    



$(document).ready(function(){
  var csrfHash=&quot;8e1697f44a0589e2301d9e20f70b0908&quot;;
  $('.cari').click(function(){
    $.ajax({
      url:'https://inixindojogja.com/api/index.php/user/cari',
      method:&quot;POST&quot;,
      dataType:'JSON',
      data:{
        q:$('input[name=table_search]').val(),
        nama_token:csrfHash
      },
      success:function(a){
        $('tbody').html(a.body); csrfHash=a.csrfHash;
      }
    });
  });
});

                
                
            </value>
      <webElementGuid>fdf76f46-ec18-436a-98bf-c03ac7872f99</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;skin-blue sidebar-mini&quot;]/div[@class=&quot;wrapper&quot;]/div[@class=&quot;content-wrapper&quot;]</value>
      <webElementGuid>68226610-c4f5-438a-982d-1638981185ca</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Information'])[1]/following::div[1]</value>
      <webElementGuid>e3a063c6-f34e-41c2-900b-1d4420717690</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Warning'])[1]/following::div[1]</value>
      <webElementGuid>67654ba5-a509-4f36-945c-4c6f5a0e3d31</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div</value>
      <webElementGuid>715bf77f-ec53-46cd-9627-010de5c9c674</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
                
                
                
                    
    
        Responsive Hover Table
        
            
                
                    
                
                
                
                    
                
            
        
    
    
    
        
            
                    Username
                    Nama
                    Level
                    LastLogin
                    LastIP
                    Token
                    Tools
                
            
            
                
operator@oti.com
OperatorApliaksi
2
2023-05-09 14:03:50
182.4.103.211
93de4ce4fb4c9318f0c5baca0cd47635f7f026fbbc6b0b0939020ff4c1191b50456a71014ba16481744efaabea513b21015256a5035a9ad5990a3bd4f07fa3edrF_fPs56j7RoOJmdw7YiEdCCtvPzbCUwCva1hZtZUw8eUVAnM2DsAIiPtkeHztMy





muhammadyasin@oti.com
Muhammad Yasin
2
2023-05-09 13:38:42







Marvin@oti.com
Marvin
2
2023-05-09 13:40:26







wiuwiu@oti.com
wiuwiu
2
2023-05-09 13:48:14







sultonmukarobin@oti.com
Sulton Daud Ul Mukarobin
2
2023-05-09 13:48:21







muhammad.yasin@oti.com
Muhammad Yasin
2
2023-05-09 13:55:16







mugen@oti.com
MugenKun
2
2023-05-09 13:57:35







puterishofro@oti.com
Puteri Shofro
2
2023-05-09 13:59:33







yakobusarisarvanto@oti.com
Aris
2
2023-05-09 14:02:08







heleh@oti.com
heleh
2
2023-05-09 14:02:35






            
        
    
    
    
    12>    



$(document).ready(function(){
  var csrfHash=&quot;8e1697f44a0589e2301d9e20f70b0908&quot;;
  $(&quot; , &quot;'&quot; , &quot;.cari&quot; , &quot;'&quot; , &quot;).click(function(){
    $.ajax({
      url:&quot; , &quot;'&quot; , &quot;https://inixindojogja.com/api/index.php/user/cari&quot; , &quot;'&quot; , &quot;,
      method:&quot;POST&quot;,
      dataType:&quot; , &quot;'&quot; , &quot;JSON&quot; , &quot;'&quot; , &quot;,
      data:{
        q:$(&quot; , &quot;'&quot; , &quot;input[name=table_search]&quot; , &quot;'&quot; , &quot;).val(),
        nama_token:csrfHash
      },
      success:function(a){
        $(&quot; , &quot;'&quot; , &quot;tbody&quot; , &quot;'&quot; , &quot;).html(a.body); csrfHash=a.csrfHash;
      }
    });
  });
});

                
                
            &quot;) or . = concat(&quot;
                
                
                
                    
    
        Responsive Hover Table
        
            
                
                    
                
                
                
                    
                
            
        
    
    
    
        
            
                    Username
                    Nama
                    Level
                    LastLogin
                    LastIP
                    Token
                    Tools
                
            
            
                
operator@oti.com
OperatorApliaksi
2
2023-05-09 14:03:50
182.4.103.211
93de4ce4fb4c9318f0c5baca0cd47635f7f026fbbc6b0b0939020ff4c1191b50456a71014ba16481744efaabea513b21015256a5035a9ad5990a3bd4f07fa3edrF_fPs56j7RoOJmdw7YiEdCCtvPzbCUwCva1hZtZUw8eUVAnM2DsAIiPtkeHztMy





muhammadyasin@oti.com
Muhammad Yasin
2
2023-05-09 13:38:42







Marvin@oti.com
Marvin
2
2023-05-09 13:40:26







wiuwiu@oti.com
wiuwiu
2
2023-05-09 13:48:14







sultonmukarobin@oti.com
Sulton Daud Ul Mukarobin
2
2023-05-09 13:48:21







muhammad.yasin@oti.com
Muhammad Yasin
2
2023-05-09 13:55:16







mugen@oti.com
MugenKun
2
2023-05-09 13:57:35







puterishofro@oti.com
Puteri Shofro
2
2023-05-09 13:59:33







yakobusarisarvanto@oti.com
Aris
2
2023-05-09 14:02:08







heleh@oti.com
heleh
2
2023-05-09 14:02:35






            
        
    
    
    
    12>    



$(document).ready(function(){
  var csrfHash=&quot;8e1697f44a0589e2301d9e20f70b0908&quot;;
  $(&quot; , &quot;'&quot; , &quot;.cari&quot; , &quot;'&quot; , &quot;).click(function(){
    $.ajax({
      url:&quot; , &quot;'&quot; , &quot;https://inixindojogja.com/api/index.php/user/cari&quot; , &quot;'&quot; , &quot;,
      method:&quot;POST&quot;,
      dataType:&quot; , &quot;'&quot; , &quot;JSON&quot; , &quot;'&quot; , &quot;,
      data:{
        q:$(&quot; , &quot;'&quot; , &quot;input[name=table_search]&quot; , &quot;'&quot; , &quot;).val(),
        nama_token:csrfHash
      },
      success:function(a){
        $(&quot; , &quot;'&quot; , &quot;tbody&quot; , &quot;'&quot; , &quot;).html(a.body); csrfHash=a.csrfHash;
      }
    });
  });
});

                
                
            &quot;))]</value>
      <webElementGuid>d2192334-7b6a-402d-80b8-b0e7df57ef56</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
